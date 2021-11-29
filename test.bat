@Echo Off
echo Building...
cargo build

echo Testing store...
type test-stdin.json | .\target\debug\docker-credential-redis.exe store
pause

echo Testing get...
echo https://index.docker.io/v1 | .\target\debug\docker-credential-redis.exe get
pause

echo Testing get with unknown value...
echo https://index.docker.io/v2 | .\target\debug\docker-credential-redis.exe get
pause

echo Testing erase...
echo https://index.docker.io/v1 | .\target\debug\docker-credential-redis.exe erase
pause

echo Testing get after erase...
echo https://index.docker.io/v1 | .\target\debug\docker-credential-redis.exe get
pause