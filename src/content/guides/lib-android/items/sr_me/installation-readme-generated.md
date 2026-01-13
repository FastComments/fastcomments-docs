Додајте FastComments SDK у `build.gradle.kts` фајл ваше апликације:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Проверите да ли имате Repsy репозиторијум у `settings.gradle.kts` вашег пројекта:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // други репозиторијуми...
    }
}
```