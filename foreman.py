# Foreman - python script to build the apps in the apps folder and move them onto the rootfs

print("👷‍ Foreman v1.0.0 - Building Hydro Apps")

import os
import shutil
import subprocess

persist = False
forceRebuild = False

# check if the argument --persist is passed
if "--persist" in os.sys.argv:
    persist = True
    print("|-🔒 Persistence mode enabled: won't stop if there's a build failure")

if "--force-rebuild" in os.sys.argv:
    forceRebuild = True
    print("|-🧱 Forcing a rebuild of all apps")

# Get a list of folders in the apps directory
apps = os.listdir("apps")

print(f"|-🔎 Found {len(apps)} apps.")

# Loop through the apps and build them
print("|-🔨 Building apps...")

for app in apps:
    # see if the files in src directory are newer than ./main.wasm (if it exists)
    # if they are, then `make clean` and `make`

    wasm_file_age = os.path.getmtime(f"apps/{app}/main.wasm") if os.path.exists(f"apps/{app}/main.wasm") else 0
    src_files = os.listdir(f"apps/{app}/src")
    extern_files = []

    # see if there's a extern_deps.foreman file in the app directory
    if os.path.exists(f"apps/{app}/extern-deps.foreman"):
        with open(f"apps/{app}/extern-deps.foreman", "r") as f:
            extern_deps = f.read().split("\n")
            # the extern_deps are paths to files that the app should be rebuilt if they change
            # we need to check if they're newer than the wasm file
            # we'll add them to the src_files list if they are
            for dep in extern_deps:
                if os.path.exists(dep):
                    extern_files.append(dep)
                else:
                    print(f"  |-❌ Error: extern_dep {dep} not found for {app}.")
                    if not persist:
                        exit(1)
                    else:
                        apps.remove(app)

    # see if there's a .foremanignore file in the app directory
    if os.path.exists(f"apps/{app}/.foremanignore"):
        with open(f"apps/{app}/.foremanignore", "r") as f:
            ignore_files = f.read().split("\n")
            src_files = [file for file in src_files if file not in ignore_files]

    src_files_age = max([os.path.getmtime(f"apps/{app}/src/{file}") for file in src_files])
    extern_files_age = max([os.path.getmtime(file) for file in extern_files]) if len(extern_files) > 0 else 0

    if (src_files_age > wasm_file_age) or (extern_files_age > wasm_file_age) or forceRebuild:
        print(f" |-🔧 Building {app}...")
        if os.path.exists(f"apps/{app}/main.wasm"):
            subprocess.run(f"cd apps/{app} && make clean", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        make_process = subprocess.Popen(f"cd apps/{app} && make", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        stdout, stderr = make_process.communicate()
        if make_process.returncode != 0:
            print(f"  |-❌ Error building {app}.")

            # write the stdout and stderr to apps/{app}/build-error.foreman.generated
            with open(f"apps/{app}/build-error.foreman.generated", "w", encoding="utf-8") as f:
                f.write(stdout.decode(errors="ignore"))
                f.write(stderr.decode(errors="ignore"))
            
            # remove it from the list of apps to copy to rootfs
            if not persist:
                exit(1)
            else:
                apps.remove(app)
            
        else:
            print(f"  |-✅ Successfully built {app}.")
    else:
        print(f" |-💾 Re-using cached build of {app}...")

print("|-🚚 Moving apps to rootfs...")

# clear rootfs/Applications folder, if it exists
if os.path.exists("rootfs/Applications"):
    # delete every file in the folder, but not the folder itself
    for file in os.listdir("rootfs/Applications"):
        os.remove(f"rootfs/Applications/{file}")
else:
    # create the folder
    os.makedirs("rootfs/Applications")

# Move the built apps to the rootfs
for app in apps:
    # convert the data from the wasm file into like [0x00, 0x01, 0x02, ...]
    with open(f"apps/{app}/main.wasm", "rb") as f:
        data = f.read()
        data = [byte for byte in data]
    # write this as a string to the rootfs at Applications/{app}.hydro
    with open(f"rootfs/Applications/{app}.hydro", "w") as f:
        f.write(str(data))

print(" |-🎉 Done!")
