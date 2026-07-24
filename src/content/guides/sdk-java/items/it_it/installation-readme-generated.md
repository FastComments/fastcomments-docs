### Maven

Aggiungi il repository Repsy al POM del tuo progetto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Quindi aggiungi le dipendenze di cui hai bisogno:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Aggiungi il repository Repsy al file **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Contenuto della Libreria

Questa libreria contiene tre moduli. Il client API generato, la libreria core Java che contiene utility scritte a mano per semplificare l'uso dell'API, e il modulo `pubsub` che è una libreria per iscriversi ai feed di cambiamento.

- [Documentazione della Libreria Client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentazione della Libreria Core, Inclusi Esempi SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentazione della Libreria PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API Pubbliche vs API Sicure

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi` e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua chiave API, e la `PublicApi` contiene metodi che possono essere chiamati direttamente da un browser/dispositivo mobile/etc senza autenticazione.

La `ModerationApi` fornisce una suite completa di API di moderazione in tempo reale e veloci. Ogni metodo della `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.