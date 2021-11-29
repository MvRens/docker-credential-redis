## Docker credential helper for Redis
**WARNING: USE AT YOUR OWN RISK**

This is not in any way a secure or recommended way to store Docker credentials. Use one of the official [credential stores](https://docs.docker.com/engine/reference/commandline/login/#credentials-store) instead.

This Docker credential helper stores the credentials in a Redis instance, currently hardcoded as localhost and database #15.

It was made to get rid of the warning when using the default store while playing nice with CI deployments. Again, it is not secure at all. You are at least recommended to not persist the Redis database to disk.