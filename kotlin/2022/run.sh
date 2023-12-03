#!/bin/bash

# Run the following to download dependencies into target/dependency directory:
# mvn dependency:copy-dependencies

classpath=$(find target/dependency -type f -name '*.jar' | paste -sd ':' -)

# compile
kotlinc -cp "$classpath" -d target/classes $(find . -type f -name '*.kt') $(find . -type f -name '*.kts') $(find . -type f -name '*.java')

# run
kotlin -cp "$classpath" -cp target/classes org.junit.platform.console.ConsoleLauncher --fail-if-no-tests --scan-classpath
