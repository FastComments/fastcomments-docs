Додајте FastComments SDK у `build.gradle.kts` датотеку ваше апликације:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Уверите се да имате Repsy репозиторијум у `settings.gradle.kts` датотеци вашег пројекта:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // остали репозиторијуми...
    }
}
```