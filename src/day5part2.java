package com.adventofcode.day5.part2;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

/**
 * I had to switch to Java for this problem because the regex crate did not support back references.
 * I could have come up with some coded solution, but regex is much quicker.
 */
public class MainTest {

	public static void main(String[] args) {
        BufferedReader reader;
		try {
			String pattern1 = "([a-z][a-z]).*\\1";
			String pattern2 = "([a-z]).{1}\\1";
			String line;
			reader = new BufferedReader(new FileReader("input.txt"));
			int count = 0;
			while ((line = reader.readLine()) != null) {
				String p1 = line.replaceFirst(pattern1, "");
				String p2 = line.replaceFirst(pattern2, "");
				int len = line.length();
				if (p1.length() < len && p2.length() < len) {
					count++;
				}
			}
			System.out.println("Matches = " + count);
		} catch (IOException e) {
			e.printStackTrace();
		} finally {
			try {
				if (reader != null) reader.close();
			} catch (IOException e) {
				e.printStackTrace();
			}
		}
	}
}
