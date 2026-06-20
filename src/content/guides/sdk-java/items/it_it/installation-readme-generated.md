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

Quindi aggiungi le dipendenze necessarie:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
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
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Contenuti della libreria

Questa libreria contiene tre moduli. Il client API generato, la libreria Java core che contiene utilità scritte a mano per rendere più semplice il lavoro con l'API, e il modulo `pubsub` che è una libreria per iscriversi ai feed di cambiamento.

- [Documentazione libreria client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentazione libreria core, inclusi esempi SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentazione libreria PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### API Pubbliche vs Protette

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi`, e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua API key, e `PublicApi` contiene metodi che possono essere eseguiti direttamente da un browser/dispositivo mobile/etc senza autenticazione.

La `ModerationApi` alimenta la dashboard dei moderatori. Contiene metodi per la moderazione dei commenti (elenco, conteggio, ricerca, log e esportazione), azioni di moderazione (rimuovere/ripristinare, segnala, impostare lo stato di revisione/spam/approvazione, voti e riaprire/chiudere il thread), ban (vietare di commentare, annullare un ban, riepiloghi pre-ban, stato e preferenze del ban, e conteggi utenti bannati), e badge & fiducia (assegnare/rimuovere un badge, badge manuali, ottenere/impostare il fattore di fiducia, e profilo interno utente). Ogni metodo di `ModerationApi` accetta un parametro `sso` in modo che la chiamata possa essere eseguita per conto di un moderatore autenticato tramite SSO.