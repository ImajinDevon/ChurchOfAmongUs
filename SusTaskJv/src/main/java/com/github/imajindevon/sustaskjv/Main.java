package com.github.imajindevon.sustaskjv;

import org.jetbrains.annotations.NotNull;

import java.io.IOException;
import java.net.URL;
import java.util.regex.Pattern;

public final class Main {
    private static final Pattern LINE_SEPARATOR_PATTERN = Pattern.compile("\r?\n|\r");
    private static final Pattern MARKDOWN_STRIP_PATTERN = Pattern.compile("^\s#");

    private Main() {
    }

    /**
     * Print the scripture with removed markdown elements.
     *
     * @param args the args
     * @throws IllegalArgumentException if arguments are supplied
     */
    public static void main(@NotNull String[] args) {
        if (args.length != 0) {
            throw new IllegalArgumentException("This program takes no arguments.");
        }

        try {
            URL url = Main.class.getClassLoader().getResource("Scripture.md");

            if (url == null) {
                throw new IllegalArgumentException("The JAR must be modified, because it does not contain the " +
                                                       "scripture.");
            }

            String content = (String) url.getContent(new Class[] { String.class });

            for (String line : LINE_SEPARATOR_PATTERN.split(content)) {
                System.out.println(MARKDOWN_STRIP_PATTERN.matcher(line).replaceAll(""));
            }
        } catch (IOException | ClassCastException exception) {
            throw new TypeNotPresentException("The JAR must be modified, because the scripture does not contain a " +
                                                  "valid string.", exception);
        }
    }
}