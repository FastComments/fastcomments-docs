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
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане помоћне алате да би рад са API-јем био лакши, и `pubsub` модул који је библиотека за претплату на канале промена.

- [Документација за API клијент библиотеку](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација за Core библиотеку, укључујући SSO примере](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација за PubSub библиотеку](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клијента постоје две класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи API позиве који се могу извршити директно из претраживача/мобилног уређаја/итд без аутентификације.