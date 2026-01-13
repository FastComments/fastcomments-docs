Об'єкт `NotificationCount` представляє собою кількість непрочитаних сповіщень та метадані для користувача.

Якщо немає непрочитаних сповіщень, для користувача не існуватиме `NotificationCount`.

`NotificationCount` об'єкти створюються автоматично і не можуть бути створені через API. Термін їх дії закінчується через один рік.

Ви можете очистити кількість непрочитаних сповіщень користувача, видаливши його `NotificationCount`.

Структура об'єкта `NotificationCount` виглядає так:

[inline-code-attrs-start title = 'Структура NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // ідентифікатор користувача
    count: number
    createdAt: string // рядок дати
    expireAt: string // рядок дати
}
[inline-code-end]

---