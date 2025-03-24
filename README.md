## Assignment
I hope I did not misunderstand the assignment, there wasn't much to go on

## Tech stack
- Clap: A very popular CLI parsing library to easily parse CLI parameters
- Tokio: De-facto standard runtime for async workloads. Obviously this exercise can be done without any threading, but I like the 'message passing' architecture the assignment hints at. At first I started using standard hardware threads, thinking it was simpler, but I did not really pan out.
- Broadcast channels: One of the reasons I switched to tokio was that I could use ergonomic broadcast channels, allowing for more than one subscriber.

## Testing
I have some unit testing for the parsing and IFF evaluation, 

## To Do
- CI
- Integration test
- Run no_alloc
- Add statistics viewer
- Configure buffer size
