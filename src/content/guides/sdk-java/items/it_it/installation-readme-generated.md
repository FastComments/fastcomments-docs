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
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Aggiungi il repository Repsy al file build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Contenuto della libreria

Questa libreria contiene tre moduli. Il client API generato, la libreria core Java che contiene utility scritte a mano per facilitare il lavoro con l'API, e il modulo `pubsub` che è una libreria per iscriversi ai feed di cambiamento.

- [Documentazione libreria client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentazione libreria Core, inclusi esempi SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentazione libreria PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API Pubbliche vs Protette

Per il client API, ci sono due classi, `DefaultApi` e `PublicApi`. `DefaultApi` contiene metodi che richiedono la tua chiave API, e `PublicApi` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione.