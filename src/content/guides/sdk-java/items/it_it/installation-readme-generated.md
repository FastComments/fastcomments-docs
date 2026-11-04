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
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (include SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (per eventi live) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Aggiungi il repository Repsy al tuo file build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (include SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (per eventi live)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Contenuto della Libreria

Questa libreria contiene tre moduli: il client API generato, la libreria Java core che contiene utilità scritte a mano
per semplificare l'interazione con l'API, e il modulo `pubsub` che è una libreria per sottoscriversi ai feed di modifiche.

- [Documentazione della API Client Library](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentazione della Core Library, Inclusi Esempi SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentazione della PubSub Library](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API Pubbliche vs API Protette

Per il client API, esistono tre classi: `DefaultApi`, `PublicApi` e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua API key, mentre la `PublicApi` contiene metodi
che possono essere invocati direttamente da un browser/dispositivo mobile/ecc senza autenticazione.

La `ModerationApi` fornisce un'ampia suite di API di moderazione live e veloci. Ogni metodo di `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione di FastComments.com.