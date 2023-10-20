## Запуск

docker-compose up --build

## Подключение 

```
docker-compose exec -ti tarantool console
```

## Вызов хранимой функции
```
box.schema.func.call('app.insert')
```

## Задачи
- Замерить размер space-ов

## Результаты

Вставка 100_000 plan_item MacOSx memtx
- transaction * N + println!() = 12s
- transaction * N = 8s
- transaction * 1 = 300ms

Вставка 100_000 plan_item MacOSx vinyl
- transaction * 1 = 1600ms
