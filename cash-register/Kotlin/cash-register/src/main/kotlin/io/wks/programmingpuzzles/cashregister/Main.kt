package io.wks.programmingpuzzles.cashregister

import java.lang.Exception
import java.math.BigDecimal

enum class Currency(val value: BigDecimal) {
    ZERO(BigDecimal.ZERO.setScale(2)),
    PENNY(BigDecimal("0.01")),
    NICKEL(BigDecimal("0.05")),
    DIME(BigDecimal("0.10")),
    HALF_DOLLAR(BigDecimal("0.50")),
    ONE(BigDecimal.ONE.setScale(2)),
    TWO(BigDecimal("2.00")),
    FIVE(BigDecimal("5.00")),
    TEN(BigDecimal.TEN.setScale(2)),
    TWENTY(BigDecimal("20.00")),
    FIFTY(BigDecimal("50.00")),
    ONE_HUNDRED(BigDecimal("100.00"));

    override fun toString(): String {
        return this.name.replace("_", " ")
    }
}

class CashRegister(private val contents: Array<Currency>) {

    private val sortedContents by lazy {
        contents.sortedByDescending { it.value }
    }

    fun change(price: BigDecimal, cash: BigDecimal): String {
        if (price.compareTo(cash) == 0) return "ZERO"
        if (price.compareTo(cash) == 1) return "ERROR"

        val currencies = mutableListOf<Currency>()
        var remainingChange = cash.subtract(price)

        sortedContents
            .filter { it != Currency.ZERO }
            .takeWhile { remainingChange > BigDecimal.ZERO }
            .onEach { currency ->

                while (remainingChange >= currency.value) {
                    remainingChange = remainingChange.subtract(currency.value)
                    currencies.add(currency)
                }
            }

        return currencies.sortedBy { it.name }.joinToString(separator = ",") { it.toString() }
    }

}

open class Main {
    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            val register = CashRegister(Currency.values())
            val (price, cash) = Pair(args[0], args[1])
            try {
                println(register.change(BigDecimal(price), BigDecimal(cash)))
            } catch (e: Exception) {
                print("Invalid Input (${e.message})")
            }
        }
    }
}