<?php

function handler(int $errno, string $errstr): bool
{
    throw new Exception($errstr, $errno);
}

set_error_handler("handler");

function execLine(string $line): string
{
    try {
        ob_start();
        $result = eval ("return $line;");
        $stdout = ob_get_clean();
        $string = json_encode(["ok" => ["result" => $result, "stdout" => $stdout]], JSON_PARTIAL_OUTPUT_ON_ERROR);
    } catch (\Throwable $t) {
        $stdout = ob_get_clean();
        return json_encode(["error" => ["message" => $t->getMessage(), "code" => $t->getCode(), "stdout" => $stdout ? $stdout : ""]]);
    }
    return $string;
}

while (true) {
    echo execLine(readline()) . PHP_EOL;
}