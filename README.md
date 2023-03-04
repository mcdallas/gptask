# gptask

a CLI tool to interact with chatGPT

## Installation 
``` bash
cargo install gptask
export OPENAI_API_KEY=sk-WEz... # from https://platform.openai.com/account/api-keys
```

## Usage
You will need an OpenAI API key, and to set the environment variable `OPENAI_API_KEY`.

``` bash
>>> gptask Who won the world series of poker in 1989?
The winner of the World Series of Poker in 1989 was Phil Hellmuth.
```

combine with mdcat to format markdown and code snippets

``` bash
ask() { gptask "$@" | mdcat }
```
![image](https://user-images.githubusercontent.com/15388116/222921902-649f02a6-af1e-4a10-8448-71eea0897e5a.png)
