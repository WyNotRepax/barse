<?php

array_map(function ($token) {
    if (is_array($token)) {
        return [
            "complex" => [
                "name" => token_name($token[0]),
                "content" => $token[1],
            ]
        ];
    } else {
        return ["simple" => $token];
    }
}, token_get_all(/**CODE**/));