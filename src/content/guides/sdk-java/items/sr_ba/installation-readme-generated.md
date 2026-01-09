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
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додајте Repsy репозиторијум у ваш build.gradle фајл:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Садржај библиотеке

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане помоћне алате да би рад са API-јем био једноставнији, и модул `pubsub` који је библиотека за претплату на фидове промјена.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација Core библиотеке, укључујући примјере SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Јавни и заштићени API-ји

За API клијент постоје две класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи позиве API-ја које се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.