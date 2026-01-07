Ein `NotificationCount`-Objekt repräsentiert die Anzahl ungelesener Benachrichtigungen und Metadaten für einen Benutzer.

Wenn es keine ungelesenen Benachrichtigungen gibt, existiert kein `NotificationCount` für den Benutzer.

`NotificationCount`-Objekte werden automatisch erstellt und können nicht über die API erstellt werden. Sie laufen auch nach einem Jahr ab.

Sie können den Zähler für ungelesene Benachrichtigungen eines Benutzers löschen, indem Sie seinen `NotificationCount` löschen.

Die Struktur des `NotificationCount`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'NotificationCount Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
