# Scripts

## add-job

To simplify the process of creating a new verification job, the repository contains a script to generate all necessary files and to modify the correct CMake file. To create a new job with the script, run the following from the root project directory:

```bash
./add-job <JOB_NAME>...
```

Where `JOB_NAME` is the name of the job to add. Note that multiple jobs can be added at once.

Once the files have been generated, you will have to update the new Rust file with the code for your job.

To remove a job, simply delete the files that were generated and remove the corresponding lines from `src/rust-jobs/CMakeLists.txt` and `rust-jobs.json`.
