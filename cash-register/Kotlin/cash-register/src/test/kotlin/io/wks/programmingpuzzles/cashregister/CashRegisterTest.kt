package io.wks.programmingpuzzles.cashregister

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource
import java.math.BigDecimal
import java.util.stream.Stream

internal class CashRegisterTest{

    @Test
    fun `GIVEN cash WHEN cash equals price THEN change is zero`(){
        // GIVEN
        val cash = BigDecimal.TEN

        //WHEN
        val price = BigDecimal.TEN

        // THEN
        val register = CashRegister(Currency.values())
        assertEquals("ZERO", register.change(price, cash))
    }

    @Test
    fun `GIVEN cash WHEN price is greater than cash THEN error is returned`(){
        // GIVEN
        val cash = BigDecimal.ONE

        //WHEN
        val price = BigDecimal.TEN

        // THEN
        val register = CashRegister(Currency.values())
        assertEquals("ERROR", register.change(price, cash))
    }

    @ParameterizedTest
    @MethodSource("provideCashAndPrices")
    fun `GIVEN cash WHEN cash is greater than price THEN correct change is returned`(
        cash: BigDecimal,
        price: BigDecimal,
        expectedChange: String
    ){
        val register = CashRegister(Currency.values())
        assertEquals(expectedChange, register.change(price, cash))
    }

    companion object{
        @JvmStatic
        fun provideCashAndPrices(): Stream<Arguments> = Stream.of(
            Arguments.of(BigDecimal("20"), BigDecimal("15"), "FIVE"),
            Arguments.of(BigDecimal("16.00"), BigDecimal("15.94"), "NICKEL,PENNY"),
            Arguments.of(BigDecimal("100.00"), BigDecimal("1.00"), "FIFTY,FIVE,TWENTY,TWENTY,TWO,TWO"),
        )
    }
}