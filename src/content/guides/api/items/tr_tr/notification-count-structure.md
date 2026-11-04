Bir `NotificationCount` nesnesi, bir kullanıcının okunmamış bildirim sayısını ve meta verilerini temsil eder.

Okunmamış bildirim yoksa, kullanıcı için bir `NotificationCount` bulunmaz.

`NotificationCount` nesneleri otomatik olarak oluşturulur ve API aracılığıyla oluşturulamaz. Ayrıca bir yıl sonra sona ererler.

Bir kullanıcının okunmamış bildirim sayısını, kullanıcının `NotificationCount` kaydını silerek temizleyebilirsiniz.

`NotificationCount` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'NotificationCount Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]