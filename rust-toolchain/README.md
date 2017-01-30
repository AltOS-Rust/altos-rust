# Installing Rust Components
Rustup is an installer for the Rust programming language and provides a simple way of installing both the stable and nightly versions of Rust as well as a simple command to switch between the two version.

At the time of writing this, the following instructions for setting up the toolchain has been tested on the following platforms:

- Ubuntu 16.04 (and OS's based on 16.04)
- OS X El Capitan

*All currently installed versions of Rust must be uninstalled before attempting to install Rustup.*

#### To uninstall Rust:

There are two potential locations that Rust originally installed.  
Use the appropriate uninstall command.
```bash
  sudo /usr/lib/rustlib/uninstall.sh
```  
or  
```bash
  sudo /usr/local/lib/rustlib/uninstall.sh
```

#### To install [Rustup](https://www.rustup.rs/):  
```bash
  curl https://sh.rustup.rs -sSf | sh
```

Put the following at the bottom of your .zshrc or .bashrc:  
```bash
  export PATH=${HOME}/.cargo/bin:$PATH
```
  
To change between Rust version (stable/nightly):  
```bash
  rustup default stable
```  
```bash
  rustup default nightly
```  

Rustup defaults to stable so the first time that
the nightly command is ran, expect it to install the nightly version of Rust.

#### Install [xargo](https://github.com/japaric/xargo)
Xargo is Rust package for cross compiling programs to a specific target architecture.  
For more information, follow the above link.

To install xargo, run the following command:  
```bash
  cargo install xargo
```

#### Install rust-src
This is a component dependency for compiling a Rust project to ARM.    
```bash
  rustup component add rust-src
```  

# Installing Compiler Components
The following packages will need to be installed in order to compile/debug a Rust or C program for the target ARM architecture.

##### Debian-based distro
```bash
  sudo apt update
```  
```bash
  sudo apt install gcc-arm-none-eabi
```  
```bash
  sudo apt install gdb-arm-none-eabi
```  

##### OS X

brew install gcc-arm-none-eabi-48

# Installing/Using USB Connectivity and Debugger Programs

For linking to the STM32F0xx and doing on board debugging, there are two options: [OpenOCD](http://openocd.org/) and [Stlink](https://github.com/texane/stlink)   

From our experience thus far, while OpenOCD is easier to install, Stlink is easier to use.  
For more information, and to research the differences between the two, please follow the respective link above.

The install instructions for each are provided below. Note that there are not OS X instructions provided here. To check for OS X install instructions, follow the OpenOCD link above.

## OpenOCD

### Installing OpenOCD
```bash
  sudo apt update
```  
```bash
  sudo apt install openocd
```

### Using OpenOCD
*First ensure the board is successfully connected via USB before running the following commands*

To open a connection to the STM32F042 board open a terminal shell and run the following command:    
```bash 
  openocd -f /usr/share/openocd/scripts/board/st_nucleo_f0.cfg
```

To connect with GDB, open another terminal shell, and run the following commands:  
```bash
  arm-none-eabi-gdb
```  
```bash
  target remote :3333
```
 
- Port 3333 is the default for OpenOCD.  
- For opening GDB with a specific binary file, please see the build section below.

## Stlink
### Installing STLink

##### OS X

Mac, FreBSD, and Arch there is a prebuilt package. Follow the Stlink link above for install instructions.

#### For Debian, Stlink must be compiled from source.
The following is a summary from the Stlink [compiling manual](https://github.com/texane/stlink/blob/master/doc/compiling.md)

##### Debian-based distro

```bash
  sudo apt update
```
```bash
  sudo apt install build-essential cmake libusb-1.0
```

May need to install the gcc compiler if not already installed.

- To check run:  
```bash
  gcc --version
```  
- To install run:  
```bash
  sudo apt install gcc
```  

#####Cloning the stlink repo:
```bash
  git clone https://github.com/texane/stlink.git
```

##### Compiling Stlink source code:
From the root of the Stlink source directory  
```bash
  make release
```   
```bash
  make debug
```  
```bash
  cd build
```  
```bash
  cmake -DCMAKE_BUILD_TYPE=Debug ..
```  
```bash
  make
```

To install system-wide (recommended):
```bash
  cd Release
```  
```bash
  sudo make install
```
  
If installing system-wide (i.e., with the above command) the dynamic library cache needs to be updated with the following commmand:  
```bash
  cd && sudo ldconfig
```

To install to a user directory instead:  
```bash
  cd Release
```  
```bash
  make install DESTDIR=$HOME
```

### Using Stlink
*First ensure the board is successfully connected via USB before running the following commands*

To open a connection to the STM32F042 board open a terminal shell and run the following command:    
```bash
  st-util
```

*If there are errors with running this command, make sure the* ```ldconfig``` *command above was done correctly.*
  
To connect with GDB, open another terminal shell, and run the following commands:  
```bash
  arm-none-eabi-gdb
```  
```bash
  target remote :4242
```

- Port 4242 is the default for Stlink.  
- For opening GDB with a specific binary file, please see the build section below.

Building and Running Programs
=============================
There are two included test programs, one in C and the other in Rust. These are just to make sure that the board is connected properly to the system, and to GDB.

The following is quick instructions on loading each respective program, and being able to view the source code in GDB.

The examples shown here are using Stlink.  
To use OpenOCD, replace with Stlink commands and port with the appropriate ones.

### Rust
Included in the Rust test program is a file called: ```cortex-m0.json```  
This file is needed to inform the Rust cross-compiler as to the target architecture.

In this case, for the STM32F042 board.
 
From the ```Rust_Test_Program``` directory, run the following commands:  
```bash
  xargo build --target cortex-m0
```  
*Notice, the ```.json``` extension is not included.*  

 From one terminal shell:  
```bash
  st-util
```

From another terminal shell:  
```bash
  arm-none-eabi-gdb target/cortex-m0/debug/altos_rust
```  
```bash
  target remote :4242
```  
```bash
  load
  continue
  continue
```  

*If the first load fails*, just run it again with: ```load```  
*If successful*, in the terminal that ```st-util``` was ran in, should see a message ending in ```jolly good!```

Next, in the GDB terminal, run: ```lay src```  
The source code should be visible.

**The cortex-m0.json file will be needed in for every project compiled to the STM32F042 board**  

Run ```xargo clean``` to remove all target files.

### On a side note:
Although it is possible to connect to gdb without giving the binary, it will not be possible to view the source code without giving gdb the path to the binary file.  
Keep this in mind should you run ```lay src``` and there is no source code available.
  
Also, the ```load``` command only needs to be used when flashing a new program to the board.
