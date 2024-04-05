# AutoEmbed
This tool has a very very small use case.

### Explanation:
I want to do some small embedded things, I was going to use probe-rs and its family of tools to flash and use the embedded board.
The issue is that I work in WSL and as such for some reason, their tooling doesn't recognise my chip through WSL, stating the debug probe cannot be created (whatever that means).
So I have set up a VM box on my machine that can talk to it. and via rsync and ssh, this tool will run the embed and other tool functions for me.

