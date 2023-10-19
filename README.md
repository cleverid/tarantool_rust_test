## Запуск

docker-compose up --build

## Подключение 

```
docker-compose exec -ti tarantool console
```

## Вызов хранимой функции
```
box.schema.func.call('app.easy')
box.schema.func.call('app.easy2')
```
