# You better do as Barbara tells you!

Barbara is a Gaussian-Process based efficient experimental design server. Given a set of experiment spaces, she will dictate which experiments should be performed in order to reduce variance in the modelled space.

This could, for example, be employed purely in-silico to run a minimal set of digital twins, or it could be run in hybrid, where the server is polled by a lab experimentalist.

A more concrete example would be running the server to spit out deep learning experiment configurations to facilitate efficient hyperparameter optimisation.

 - parameter spaces to be explored
 - defined by parameters and bounds
 - each space defined in SQLite database - Spaces and Parameters tables
 - completed jobs are recorded along with results in the Jobs and Results tables

# Docker and containerisation
Barbara lives in a docker container. Communicate with her via port 8080 from other containers. This is set up such that you would have multiple containers running experiments, each asking Barbara for instructions as they complete each run.

# Reading
 - https://www.tutorialworks.com/container-networking/
 - https://github.com/nestordemeure/friedrich