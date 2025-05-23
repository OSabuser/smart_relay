# Руководство пользователя консольной утилиты smart_relay

> Программа предназначена для управления и контроля за состоянием восемнадцати реле, которые расположены на устройстве `SmRelay Home`

Исполняемый файл программы расположен в корневой директории пользователя:
```bash
/home/user
```

## Возможности программы

Программа поддерживает два типа команд:

```bash
# Установка состояния @state на группу @range реле, где
# @range: конкретный номер реле (1 - 18), комбинация нескольких реле, 
# например 1,3,11,14,17,18 или комбинация с диапазонами реле: 1,3-7,9,14-18
# @state: требуемое состояние реле = on, off
./smart_relay set-state [range] [state]

# Получение состояния группы @range реле
# @range: конкретный номер реле (1 - 18), комбинация нескольких реле, 
# например 1,3,11,14,17,18 или комбинация с диапазонами реле: 1,3-7,9,14-18
./smart_relay get-state [range]

# Получение справки
user@roc-rk3399-pc:~$ ./smart_relay help
Check and control relay states

Usage: smart_relay <COMMAND>

Commands:
  get-state  Get (relay_range) relays state
  set-state  Set (relay_range) relays state to (state)
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Управление состоянием реле

- Управление состоянием определённого реле
```bash
user@roc-rk3399-pc:~$ ./smart_relay set-state 4 on
; Pushed relay states
; 2025-05-23T09:35:36.019003027+00:00
[RELAYS]
4=ON
```
- Управление смешанной группой реле
```bash
# Комбинация нескольких реле
# Включение реле: 2,4,5,7
user@roc-rk3399-pc:~$ ./smart_relay set-state 7,2,5,4 on
; Pushed relay states
; 2025-05-23T09:36:13.625935219+00:00
[RELAYS]
2=ON
4=ON
5=ON
7=ON

# Комбинация нескольких реле с диапазонами
# Включение реле: 1,2,3,4,5,6,7,13,14,15,16
user@roc-rk3399-pc:~$ ./smart_relay set-state 2-7,13,14-16,1 on
; Pushed relay states
; 2025-05-23T09:37:02.566768725+00:00
[RELAYS]
1=ON
2=ON
3=ON
4=ON
5=ON
6=ON
7=ON
13=ON
14=ON
15=ON
16=ON
```
- Управление всеми реле
```bash
# Выключение всех доступных реле
user@roc-rk3399-pc:~$ ./smart_relay set-state 1-18 off
; Pushed relay states
; 2025-05-23T09:39:02.598287118+00:00
[RELAYS]
1=OFF
2=OFF
3=OFF
4=OFF
5=OFF
6=OFF
7=OFF
8=OFF
9=OFF
10=OFF
11=OFF
12=OFF
13=OFF
14=OFF
15=OFF
16=OFF
17=OFF
18=OFF
```

### Получение статуса реле 

- Получение состояния определённого реле
```bash
# Получение статуса работы 7 реле
user@roc-rk3399-pc:~$ ./smart_relay get-state 7
; Fetched relay states
; 2025-05-23T09:40:52.006180306+00:00
[RELAYS]
7=ON
```
- Получение состояния смешанной группы реле
```bash
# Получение статуса работы 1,3,5,7,9,11 реле
user@roc-rk3399-pc:~$ ./smart_relay get-state 1,3,5,7,9,11
; Fetched relay states
; 2025-05-23T09:42:40.119587034+00:00
[RELAYS]
1=OFF
3=OFF
5=ON
7=ON
9=OFF
11=OFF
```
- Получение состояния всех реле 
```bash
user@roc-rk3399-pc:~$ ./smart_relay get-state 1-18
; Fetched relay states
; 2025-05-23T09:45:09.797581416+00:00
[RELAYS]
1=OFF
2=ON
3=OFF
4=ON
5=ON
6=OFF
7=ON
8=OFF
9=OFF
10=OFF
11=OFF
12=OFF
13=OFF
14=OFF
15=OFF
16=OFF
17=OFF
18=OFF
```

### Перенаправление статуса работы программы в файл

Программа завершается выводом в `stdout` информации о результате выполнения той, или иной команды. Информация выводится в формате сходным с `ini`. При необходимости анализа результатов выполнения программы, можно перенаправить вывод в файл:

```bash
user@roc-rk3399-pc:~$ ./smart_relay get-state 1,3,5,7,9,11 > op_status.ini
user@roc-rk3399-pc:~$ cat op_status.ini 
; Fetched relay states
; 2025-05-23T09:52:05.682261609+00:00
[RELAYS]
1=OFF
3=OFF
5=ON
7=ON
9=OFF
11=OFF
``` 




