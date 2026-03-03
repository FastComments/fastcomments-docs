[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO verwendet HMAC-SHA256-Verschlüsselung als Mechanismus zur Implementierung von SSO. Zuerst gehen wir die Gesamtarchitektur durch, liefern Beispiele und detaillierte Schritte.

Es gibt außerdem Dokumentation zum Migrationsprozess von anderen Anbietern mit ähnlichen SSO-Mechanismen und zu den Unterschieden.

Der Ablauf sieht folgendermaßen aus:

<div class="screenshot white-bg">
    <div class="title">Sicherer SSO-Ablauf</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Sicheres SSO-Diagramm" />
</div>

Da Secure SSO Full-Stack-Entwicklung erfordert, sind funktionsfähige Beispielimplementierungen in Java/Spring, NodeJS/Express und reinem PHP derzeit <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">auf GitHub</a>.

Obwohl wir in dem NodeJS-Beispiel ExpressJS und im Java-Beispiel Spring verwenden, sind in diesen Laufzeitumgebungen für die Implementierung von FastComments SSO keine Frameworks/Bibliotheken erforderlich — die nativen Crypto-Pakete reichen aus.

Sie müssen keine neuen API-Endpunkte für FastComments SSO schreiben. Verschlüsseln Sie einfach die Informationen des Benutzers mit Ihrem geheimen Schlüssel und übergeben Sie das Payload an das Kommentar-Widget.

#### API-Geheimschlüssel abrufen

Ihren API-Geheimschlüssel können Sie von <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">dieser Seite</a> abrufen. Sie finden diese Seite außerdem, indem Sie zu "My Account" gehen, auf die Kachel "API/SSO" klicken und dann "API-Geheimschlüssel abrufen" auswählen.

#### Parameter des Kommentar-Widgets

Die hochrangige API-Dokumentation für das Kommentar-Widget finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">hier</a>.

Lassen Sie uns näher darauf eingehen, was diese Parameter bedeuten.

Das Kommentar-Widget erhält ein Konfigurationsobjekt — Sie übergeben dieses bereits, wenn Sie FastComments verwenden, um Ihre Kunden-ID (tenantId genannt) zu übermitteln.

Um SSO zu aktivieren, übergeben Sie ein neues 'sso'-Objekt, das die folgenden Parameter enthalten muss. Die Werte sollten serverseitig erzeugt werden.

- userDataJSONBase64: Die Benutzerdaten im JSON-Format, die anschließend Base64-kodiert werden.
- verificationHash: Der HMAC-SHA256-Hash, erzeugt aus UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-Zeitstempel in **Millisekunden**. Darf nicht in der Zukunft liegen oder mehr als zwei Tage in der Vergangenheit.
- loginURL: Eine URL, die das Kommentar-Widget anzeigen kann, um den Benutzer einzuloggen.
- logoutURL: Eine URL, die das Kommentar-Widget anzeigen kann, um den Benutzer auszuloggen.
- loginCallback: Wenn statt der Login-URL angegeben, eine Funktion, die das Kommentar-Widget beim Klicken auf die Login-Schaltfläche aufruft.
- logoutCallback: Wenn statt der Logout-URL angegeben, eine Funktion, die das Kommentar-Widget beim Klicken auf die Logout-Schaltfläche aufruft.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Das Benutzerobjekt

[inline-code-attrs-start title = 'Das Benutzerobjekt'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Erforderlich. Max. 1k Zeichen. **/
    id: string;
    /** Erforderlich. Max. 1k Zeichen. Hinweis: Muss eindeutig sein. **/
    email: string;
    /** Erforderlich. Max. 1k Zeichen. Hinweis: Der Benutzername darf keine E-Mail sein. Muss nicht eindeutig sein. **/
    username: string;
    /** Optional. Max. 3k Zeichen für URLs. Standard: gravatar basierend auf der E-Mail. Unterstützt 64-kodierte Bilder, in diesem Fall liegt das Limit bei 50k Zeichen. **/ 
    avatar?: string;
    /** Optional. Standard: false. **/
    optedInNotifications?: boolean;
    /** Optional. Standard: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. Max. 100 Zeichen. Dieses Label wird neben ihrem Namen angezeigt. Standardmäßig Administrator/Moderator, wenn zutreffend. **/
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
    /** Optional, Standard: true. Auf false setzen, um den Reiter "Aktivität" im Profil des Benutzers zu aktivieren. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, Standard: false. Auf true setzen, um Profilkommentare zu deaktivieren. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, Standard: false. Auf true setzen, um Direktnachrichten an diesen Benutzer zu deaktivieren. **/
    isProfileDMDisabled?: boolean;
    /** Optionale Konfiguration für Benutzer-Badges. **/
    badgeConfig?: {
        /** Array globaler Badge-IDs, die zugewiesen werden. Auf 30 Badges begrenzt. Reihenfolge wird beibehalten. **/
        badgeIds: string[];
        /** Array von Badge-IDs, die auf die aktuelle Seite (urlId) beschränkt sind. Werden nur auf der zugewiesenen Seite angezeigt. **/
        pageBadgeIds?: string[];
        /** Wenn true, ersetzt vorhandene angezeigte Badges. Globale und seitenbezogene Badges werden unabhängig voneinander überschrieben. **/
        override?: boolean;
        /** Wenn true, aktualisiert Anzeigeeigenschaften der Badges anhand der Tenant-Konfiguration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatoren und Administratoren

Für Administratoren und Moderatoren übergeben Sie die jeweiligen Flags `isAdmin` oder `isModerator` im `SSOUser`-Objekt.

#### Benachrichtigungen

Um Benachrichtigungen zu aktivieren oder zu deaktivieren, setzen Sie den Wert von `optedInNotifications` entsprechend auf `true` oder `false`. Beim ersten Laden der Seite mit diesem Wert im SSO-Payload werden die Benachrichtigungseinstellungen des Benutzers aktualisiert.

Wenn Sie zusätzlich möchten, dass Benutzer Benachrichtigungs-E-Mails für Aktivitäten auf Seiten erhalten, die sie abonniert haben (anstatt nur In-App-Benachrichtigungen), setzen Sie `optedInSubscriptionNotifications` auf `true`.

#### VIP-Benutzer & spezielle Labels

Sie können ein spezielles Label neben dem Namen des Benutzers anzeigen, indem Sie das optionale Feld "displayLabel" verwenden.

#### Nicht authentifizierte Benutzer

Um einen nicht authentifizierten Benutzer darzustellen, lassen Sie userDataJSONBase64, verificationHash und timestamp einfach unbefüllt. Stellen Sie eine loginURL bereit.

Diese Benutzer können nicht kommentieren; stattdessen wird ihnen eine Login-Nachricht angezeigt (Nachricht, Link oder Schaltfläche, abhängig von der Konfiguration).

#### Direkte Beispiele zum Serialisieren und Hashen von Benutzerdaten

Weitere Details und Beispiele finden Sie <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">hier</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">hier</a> (java) und <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">hier</a> (php).

Wir verstehen, dass jede Integration kompliziert und mühsam sein kann. Zögern Sie nicht, sich an Ihren Ansprechpartner zu wenden oder die <a href="https://fastcomments.com/auth/my-account/help" target="_blank">Support-Seite</a> zu nutzen.

---