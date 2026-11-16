### Maven

Додајте Repsy репозиторијум у POM вашег пројекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затим додајте зависности које су вам потребне:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додајте Repsy репозиторијум у вашу build.gradle датотеку:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Садржај библиотеке

Ова библиотека садржи три модула: генерисани API клијент, основну Java библиотеку која садржи ручно писане услужне функције
које олакшавају рад са API-јем, и модул `pubsub` који је библиотека за претплату на фидове промена.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација основне библиотеке, укључујући SSO примере](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Јавни наспрам заштићених API-ја

За API клијент постоје три класе: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи методе
које се могу позивати директно из прегледача/мобилног уређаја/итд. без аутентификације.

`ModerationApi` пружа опсежан скуп брзих API-ја за модерацију у реалном времену. Свака метода `ModerationApi` прихвата `sso` параметар и може се аутентификовати путем SSO-а или колачића сесије FastComments.com.