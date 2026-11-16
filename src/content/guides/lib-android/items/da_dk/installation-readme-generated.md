Tilføj FastComments SDK til din apps `build.gradle.kts`-fil:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Sørg for, at du har Repsy-repositoryet i dit projekts `settings.gradle.kts`:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // andre repositories...
    }
}
```