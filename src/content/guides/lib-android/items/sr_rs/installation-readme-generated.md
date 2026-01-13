Додајте FastComments SDK у датотеку `build.gradle.kts` ваше апликације:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Уверите се да имате Repsy репозиторијум у датотеци `settings.gradle.kts` вашег пројекта:

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