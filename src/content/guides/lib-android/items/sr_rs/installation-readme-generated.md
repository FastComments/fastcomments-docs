Додајте FastComments SDK у вашој апликацији у датотеку `build.gradle.kts`:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Уверите се да имате Repsy репозиторијум у вашој пројектној датотеци `settings.gradle.kts`:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```