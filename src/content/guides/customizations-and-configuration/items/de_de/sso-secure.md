[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO verwendet HMAC-SHA256-Verschlüsselung als Mechanismus zur Implementierung von SSO. Zuerst gehen wir die Gesamtarchitektur durch, liefern Beispiele und detaillierte Schritte.

Es gibt auch einige Dokumentationen zur Migration von anderen Anbietern mit ähnlichen SSO-Mechanismen und den Unterschieden.

Der Ablauf sieht folgendermaßen aus:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Da Secure SSO Full-Stack-Entwicklung beinhaltet, befinden sich vollständige funktionierende Codebeispiele in Java/Spring, NodeJS/Express und reinem PHP derzeit <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">auf GitHub</a>.

Obwohl wir ExpressJS im NodeJS-Beispiel und Spring im Java-Beispiel verwenden, sind in diesen Laufzeitumgebungen keine zusätzlichen Frameworks/Bibliotheken erforderlich, um FastComments SSO zu implementieren – die nativen Crypto-Pakete reichen aus.

Sie müssen keine neuen API-Endpunkte mit FastComments SSO schreiben. Verschlüsseln Sie einfach die Benutzerinformationen mit Ihrem geheimen Schlüssel und übergeben Sie die Nutzlast an das Kommentar-Widget.

#### API-Geheimschlüssel abrufen

Ihr API-Geheimschlüssel kann von <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">dieser Seite</a> abgerufen werden. Sie finden diese Seite auch, indem Sie zu Mein Konto gehen, die Kachel API/SSO anklicken und dann auf "Get API Secret Key" klicken.

#### Parameter des Kommentar-Widgets

Hochstufige API-Dokumentation für das Kommentar-Widget finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">hier</a>.

Lassen Sie uns näher erläutern, was diese Parameter bedeuten.

Das Kommentar-Widget nimmt ein Konfigurationsobjekt entgegen – dieses übergeben Sie bereits, wenn Sie FastComments verwenden, um Ihre Mandanten-ID (tenantId) zu übergeben.

Um SSO zu aktivieren, übergeben Sie ein neues "sso"-Objekt, das die folgenden Parameter enthalten muss. Die Werte sollten serverseitig erzeugt werden.

- userDataJSONBase64: Die Benutzerdaten im JSON-Format, die anschließend Base64-codiert werden.
- verificationHash: Der HMAC-SHA256-Hash, erstellt aus UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-Zeitstempel, in **Millisekunden**. Darf nicht in der Zukunft liegen oder mehr als zwei Tage in der Vergangenheit.
- loginURL: Eine URL, die das Kommentar-Widget anzeigen kann, um den Benutzer anzumelden.
- logoutURL: Eine URL, die das Kommentar-Widget anzeigen kann, um den Benutzer abzumelden.
- loginCallback: Wenn anstelle der loginURL bereitgestellt, eine Funktion, die das Kommentar-Widget aufruft, wenn auf die Login-Schaltfläche geklickt wird.
- logoutCallback: Wenn anstelle der logoutURL bereitgestellt, eine Funktion, die das Kommentar-Widget aufruft, wenn auf die Logout-Schaltfläche geklickt wird.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Das Benutzerobjekt

The User object contains the following schema:
[inline-code-attrs-start title = 'Das Benutzerobjekt'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Erforderlich. Max. 1k Zeichen. **/
    id: string;
    /** Erforderlich. Max. 1k Zeichen. Hinweis: Muss eindeutig sein. **/
    email: string;
    /** Erforderlich. Max. 1k Zeichen. Hinweis: Der Benutzername darf keine E-Mail-Adresse sein. Muss nicht eindeutig sein. **/
    username: string;
    /** Optional. Max. 3k Zeichen für URLs. Standardmäßig wird das Gravatar-Bild basierend auf der E-Mail verwendet. Unterstützt Base64-codierte Bilder; in diesem Fall beträgt das Limit 50k Zeichen. **/ 
    avatar?: string;
    /** Optional. Standardmäßig false. **/
    optedInNotifications?: boolean;
    /** Optional. Standardmäßig false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. Max. 100 Zeichen. Dieses Label wird neben dem Namen angezeigt. Standardmäßig Administrator/Moderator, wenn zutreffend. **/
    displayLabel?: string;
    /** Optional. Max. 500 Zeichen. Dies wird anstelle des Benutzernamens angezeigt. **/
    displayName?: string;
    /** Optional. Max. 2k Zeichen. Der Name des Benutzers verlinkt darauf. **/
    websiteUrl?: string;
    /** Optional. Bis zu 100 Gruppen pro Benutzer. Eine Gruppen-ID darf nicht länger als 50 Zeichen sein. **/
    groupIds?: string[];
    /** Optional. Kennzeichnet den Benutzer als Administrator. **/
    isAdmin?: boolean;
    /** Optional. Kennzeichnet den Benutzer als Moderator. **/
    isModerator?: boolean;
    /** Optional, Standard: true. Auf false setzen, um den "activity"-Tab im Profil des Benutzers zu aktivieren. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, Standard: false. Auf true setzen, um Profilkommentare zu deaktivieren. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, Standard: false. Auf true setzen, um Direktnachrichten an diesen Benutzer zu deaktivieren. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderatoren und Administratoren

Für Admins und Moderatoren übergeben Sie die jeweiligen Flags `isAdmin` bzw. `isModerator` im `SSOUser`-Objekt.

#### Benachrichtigungen

Um Benachrichtigungen zu aktivieren oder zu deaktivieren, setzen Sie den Wert von `optedInNotifications` jeweils auf `true` oder `false`. Beim ersten Laden der Seite durch den Benutzer mit diesem Wert in der SSO-Nutzlast werden seine Benachrichtigungseinstellungen aktualisiert.

Wenn Benutzer zudem E-Mail-Benachrichtigungen für Aktivitäten auf Seiten erhalten sollen, für die sie sich angemeldet haben (anstatt nur In-App-Benachrichtigungen), setzen Sie `optedInSubscriptionNotifications` auf `true`.

#### VIP-Benutzer & spezielle Labels

Sie können ein spezielles Label neben dem Namen des Benutzers anzeigen, indem Sie das optionale Feld "displayLabel" verwenden.

#### Nicht authentifizierte Benutzer

Um einen nicht authentifizierten Benutzer darzustellen, füllen Sie einfach weder userDataJSONBase64, verificationHash noch timestamp aus. Geben Sie eine loginURL an.

Diese Benutzer können nicht kommentieren und erhalten stattdessen eine Login-Nachricht (Nachricht, Link oder Schaltfläche, je nach Konfiguration).

#### Direkte Beispiele zum Serialisieren und Hashen von Benutzerdaten

Mehr Details als Beispiele finden Sie <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">hier</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">hier</a> (java) und <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">hier</a> (php).

Wir verstehen, dass jede Integration ein komplizierter und schwieriger Prozess sein kann. Zögern Sie nicht, sich an Ihren Ansprechpartner zu wenden oder die <a href="https://fastcomments.com/auth/my-account/help" target="_blank">Support-Seite</a> zu nutzen.