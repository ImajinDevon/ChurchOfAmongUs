package com.github.imajindevon.sustaskjv;

import org.jetbrains.annotations.NotNull;

import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.util.regex.Pattern;

public final class SusTaskJv {
    private static final Pattern LINE_SEPARATOR_PATTERN = Pattern.compile("\r?\n|\r");

    private SusTaskJv() {
    }

    /**
     * Strips a string of its leading header markdown.
     *
     * @param input the input
     * @return the new string
     */
    @NotNull
    public static String stripHeaderMarkdown(@NotNull String input) {
        int i = 0;
        char c;

        while (i < input.length() && ((c = input.charAt(i)) == '#' || c == ' ')) {
            i += 1;
        }
        System.out.println(input.substring(i));
        return input;
    }

    /**
     * Print the given scripture to stdout.
     * Each line is stripped of its leading header markdown.
     *
     * @param scriptureSequence the char sequence to print
     * @see #stripHeaderMarkdown(String)
     */
    public static void printScripture(@NotNull CharSequence scriptureSequence) {
        for (String string : LINE_SEPARATOR_PATTERN.split(scriptureSequence)) {
            System.out.println(stripHeaderMarkdown(string));
        }
    }

    /**
     * Print the scripture within the JAR, with each line stripped of its header markdown.
     *
     * @param args the args
     * @throws IllegalArgumentException if arguments are supplied
     * @see #printScripture(CharSequence)
     * @see #stripHeaderMarkdown(String)
     */
    public static void main(@NotNull String[] args) {
        if (args.length != 0) {
            throw new IllegalArgumentException("This program takes no arguments.");
        }

        URL url = SusTaskJv.class.getClassLoader().getResource("Scripture.md");

        if (url == null) {
            throw new IllegalArgumentException("The JAR must be modified, because it does not contain the " +
                                                   "scripture.");
        }

        String string;

        try (InputStream stream = url.openStream()) {
            string = new String(stream.readAllBytes(), StandardCharsets.UTF_8);
        } catch (IOException | ClassCastException exception) {
            throw new IllegalArgumentException("The JAR must be modified, because the scripture does not contain a " +
                                                   "valid string.", exception);
        }
        printScripture(string);
    }
}