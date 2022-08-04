
# pacman-helper

More simplified options for package management in Arch systems.
Holding back packages in arch is one of the things that users do in their systems.  
This can be due to stability issues, especially when a new package release becomes buggy.  

**pacman-helper** provides more options for this kind of scenario, in which you want to pause a package update and also it's dependencies without affecting other programs.  
This is done by checking package's unique dependencies, and printing them out to the console, thereafter the user can know which dependencies to hold back without breaking their system.  

### WARNING

Since Arch Linux is a rolling release distro, packages may acquire new dependencies. Therefore, be careful when holding back packages while running system updates.

## usage
### **Getting unique dependencies for a package**
```sh
# Replace "package-name" with the name of your package
pacman-helper get-unique-deps "package-name"
```

### **Getting other packages that share same dependencies with the package supplied**
```sh
# Replace "package-name" with the name of your package
pacman-helper get-pacs-with-same-deps "package-name"
```
