/**
* Title: Fill in the Blanks, please!
* Description: https://codegolf.stackexchange.com/questions/143352/fill-in-the-blanks-please
* Author: W.K.S
*/
import std.stdio;
import std.algorithm;
import std.range;
import std.regex;

string fillInTheBlanks(string sentence,const string[] wordPool)
{
    auto poolIndex = 0;
    while(poolIndex < wordPool.length){
    	sentence = replaceFirst(sentence,regex("[_]+"),wordPool[poolIndex++]);
    }
    
    return sentence;
}

void main()
{
    assert(
        fillInTheBlanks("I like __ because __ __ing",
        ["ice cream","it is","satisfy"]) 
           == "I like ice cream because it is satisfying");
    assert(
        fillInTheBlanks("Things _____ for those who ____ of how things work out _ Wooden",
        ["work out best","make the best","John"]) 
           == "Things work out best for those who make the best of how things work out John Wooden");
    assert(
        fillInTheBlanks("If you are ___ willing to risk _____ you will ha_o settle for the ordi_____Jim ______n",
        ["not","the usual","ve t","nary ","Roh"]) 
           == "If you are not willing to risk the usual you will have to settle for the ordinary Jim Rohn");
    assert(
        fillInTheBlanks("S____ is walking from ____ to ____ with n_oss of ___ W_____ Churchill",
        ["uccess","failure","failure","o l","enthusiasm","inston"]) 
           == "Success is walking from failure to failure with no loss of enthusiasm Winston Churchill");
    assert(
        fillInTheBlanks("If_everyone_is_thinking ____ ____ somebody_isnt_thinking G____e P____n",
        [" "," "," ","alike","then"," "," ","eorg","atto"]) 
           == "If everyone is thinking alike then somebody isnt thinking George Patton");
    assert(
        fillInTheBlanks("Pe_________e __say ____motivation does__ last Well___her doe_ bathing____thats why we rec____nd it daily _ __________lar",
        ["opl","often ","that ","nt"," neit","s","  ","omme","Zig","Zig"]) 
           == "People often say that motivation doesnt last Well neither does bathing  thats why we recommend it daily Zig Ziglar");
    writeln("Success!");   
}