---
title: "[04/01/13] Challenge #122 [Intermediate] User-Space Threading"
url: "https://old.reddit.com/r/dailyprogrammer/comments/1ceai7/040113_challenge_122_intermediate_userspace/"
---

# *(Intermediate)*: User-Space Threading

_This challenge is more coding-focused than maths-focused.*

[Threading](<http://en.wikipedia.org/wiki/Thread_(computing\)>) is a computational model that allows the execution of small sections of instructions from different sources (i.e. threads of code), one after another, that it gives users a perception of code running in parallel. It is essentially a light-weight process that can be launched, managed, or terminated by the owning process.

Your goal is to implement an *efficient* and *dynamic* user-level threading library. You may implement this in any language and on any platform, but you may *not* use any existing threading code or implementation, such as the Win32 threading code or the UNIX pthreads lib. You may call system functions (such as interrupts and signals), but again cannot defer any thread-specific work to the operating system.

The term *efficient* means that when switching the thread of execution, you must do so as quickly as possible (big bonus points for actually measuring this). The term *dynamic* means that you provide a way (through either static variables, functions, config file, etc.) to allow end-users to change how fast you switch and what kind of algorithm you use for timing.

To help you get started, try to implement the following functions: (written in C-style for clarity)

* *ThreadID CreateThread( void (*threadFunction)(void*) )* // Returns a positive, non-zero, thread ID on success. Returns 0 on failure. Starts a thread of execution of the given thread function (for those confused, this is a C-style function being passed as an argument)
* *bool DestroyThread( ThreadID givenThreadId )* // Destroys a thread of execution, based on the given thread ID

*Please direct questions about this challenge to* /u/nint22

*Subreddit news:* We (the mods) are actively working on new submissions and fixing the bot so that it posts more correctly and consistently. The next few challenges will be directly related to new subreddit features that we want to the community to try and solve with us :-)
