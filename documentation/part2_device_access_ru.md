# Device Access

## ВАЖНО: Русский язык - это не мой родной язык! Извиняюсь за ошибки!

## Что будешь узнать 
* Получить доступ для регистры.
* Арифтметики бинарные 
* Задавать конфигурацию для GPIO
* Система модулы в русте
* Получить отзыв от девайсы

## Целы

У этого главы есть две цели. Сначала будешь узнать что такое интерфейс ввода/вывода общего назначения, короче **GPIO**, их смысл и их конфигурация.
Потом будешь получить больше знания обом система модулы в русте. Мы использовать этого чтобы приготовить классификация будущих главы.

## Приготовить окружение

Как написанно в главом readme файл мы хотим создавать оператионная система которая можно адаптироваться для других платформов, если по меньшей мере cpu cortex m4.
У других девайс есть другие адресы получить доступ для них. Один вариант - написать каждый адресс где требуеться. Но представлай себя - если меняешь девайс, каждый адрес тоже надо меняться. Это кошмар! Поэтому централизируем эту информацию чтобы просто менять модул - система будет совместимый для разных девайсы.

