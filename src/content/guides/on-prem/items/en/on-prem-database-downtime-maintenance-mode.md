FastComments supports an automatic maintenance mode. If the database goes down, it can continue to serve popular comment threads.

Additionally, in maintenance mode, all comments are saved to `BACKUP_DIR`. They will be processed (checked for spam, etc) and saved once the system is back online.

It does this by, every hour, determining the top 100 most popular comment threads and caching their content on-disk. Determining the top 100 threads
is already done from pre-calculated state, so it is not a heavy periodic job.

This is completely optional, and is only enabled if `CACHE_DIR` and `BACKUP_DIR` are set. This of course makes the application nodes stateful, however it is state that
can be lost anytime without causing the application to misbehave.

Note that in maintenance mode proper authentication of comment threads cannot be done, so only threads that are safely considered public are periodically backed up.

In maintenance mode many features are not available.
