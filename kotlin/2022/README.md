# Advent of Code 2022 - Kotlin

A simple setup to test solutions for the Advent of Code challenge in Kotlin.

I am trying to keep the setup very basic to reduce the overhead when running the [Repl](https://replit.com/@nathanjent/Advent-of-Code-2022-Kotlin).

To download dependencies run the following Maven command.

    mvn dependency:copy-dependencies

Note: Replit.com will run this automatically using it's package management feature.

The following commands are used to compile and run all the tests.

    # compile
    kotlinc -cp "$classpath" -d target/classes $(find . -type f -name '*.kt') $(find . -type f -name '*.kts') $(find . -type f -name '*.java')

    # run
    kotlin -cp "$classpath" -cp target/classes org.junit.platform.console.ConsoleLauncher --fail-if-no-tests --scan-classpath
