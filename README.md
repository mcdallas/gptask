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

>>> gptask and who did he play against?
In the final heads-up match of the 1989 World Series of Poker Main
Event, Phil Hellmuth played against the legendary poker player
Johnny Chan. The two players battled for several hours before Phil 
Hellmuth was able to defeat Chan to win the championship.
```

The context of the conversation will be remembered for 15 minutes, so you can continue an existing chat. Set the environment variable `GPTASK_TTL_SECONDS` to 0 to disable this.



Combine with mdcat to format markdown and code snippets:

``` bash
ask() { gptask "$@" | mdcat }
```
![image](https://user-images.githubusercontent.com/15388116/223573359-f24046a3-1b23-422a-9cb2-b226d43611a3.png)

