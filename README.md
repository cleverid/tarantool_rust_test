# Запуск

docker-compose up --build

# Подключение 

```
docker-compose exec -ti tarantool console
conn = require('net.box').connect(3301)
conn:call('app.easy')
```

# Ошибка

```
unix/:/var/run/tarantool/tarantool.sock> conn:call('easy')
---
- error: 'Failed to dynamically load function ''easy'': /tmp/tntQkYqIg/easy.so: undefined
    symbol: easy'
...
```
