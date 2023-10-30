## TODO

v Сохранить время
- Создать агрегат и делать сохранения через него
- Передать параметр в функцию
- Разобраться с fiber_yield(), почему он останавливает поток выполнение fiber-а

## Запуск

docker-compose up --build

## Подключение 

```
docker-compose exec -ti tarantool console
```

## Вызов хранимой функции
```
box.schema.func.call('app.insert')
box.schema.func.call('app.fiber_async')
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
    - При достижении в таблице 4млн записей скорость равномерно падает до 4сек 
    - Если сохранять порциями 100 раз по 1_000 штук то результат на 40% быстрее
    - Если сохранять порциями 10 раз по 10_000 штук то результат на 50% быстрее
