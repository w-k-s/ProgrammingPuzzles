#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <set>
#include <map>
#include <algorithm>

#define INDEX(bedNumber) (bedNumber - 1)

int main()
{

    std::ifstream inputFile("input", std::ios_base::in);

    int lineNumber;

    std::set<int> assignedBedrooms;
    std::map<int, std::vector<int> > studentNumberToRequestedBedroomsMap;
    std::vector<int> bedroomDemand;

    if (inputFile.is_open()) {
        for (std::string line;getline(inputFile, line);) {
            ++lineNumber;

            std::stringstream lineStream(line);

            if (lineNumber == 1) {
                continue;
            }
            if (bedroomDemand.empty()) {

                int numberOfStudents;
                lineStream >> numberOfStudents;
                int numberOfBeds;
                lineStream >> numberOfBeds;

                bedroomDemand.resize(numberOfBeds);
                std::fill(bedroomDemand.begin(), bedroomDemand.end(), 0);

                // initialize studentNumberToRequestedBedroomsMap
                for (int i = 1; i <= numberOfStudents; i++) {
                    studentNumberToRequestedBedroomsMap[i] = std::vector<int>();
                }

                continue;
            }

            int bedroomNumber;
            lineStream >> bedroomNumber;
            int studentNumber;
            lineStream >> studentNumber;

            if (studentNumber != 0) {

                studentNumberToRequestedBedroomsMap[studentNumber].push_back(bedroomNumber);
                bedroomDemand[INDEX(bedroomNumber)]++;
            }
            else {

                // End of test case

                // Sort bedRequested in order of least in-demand to most in-demand
                for (auto& item : studentNumberToRequestedBedroomsMap) {
                    std::sort(
                        item.second.begin(),
                        item.second.end(),
                        [&bedroomDemand](int bedroomNumberLeft, int bedroomNumberRight) {
                            return bedroomDemand[INDEX(bedroomNumberLeft)] < bedroomDemand[INDEX(bedroomNumberRight)];
                        });
                }

                // Start assigning rooms to students starting with the least in-demand
                // room requested.
                for (auto item : studentNumberToRequestedBedroomsMap) {
                    for (auto bedroomNumber : item.second) {
                        auto result = assignedBedrooms.insert(bedroomNumber);
                        if (result.second) {
                            break;
                        }
                    }
                }

                std::cout << assignedBedrooms.size() << std::endl;

                assignedBedrooms.clear();
                studentNumberToRequestedBedroomsMap.clear();
                bedroomDemand.clear();
            }
        }

        inputFile.close();
    }
}