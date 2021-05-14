#!/bin/bash
GREEN='\033[0;32m'
NC='\033[0m']

echo '.DS_Store
test/
' >> .gitignore

echo '{

}
' >> default.project.json

echo '# Project Name
Lorem Ipsum Description Text

## Chapter 1
> Something

## Chapter 2
> Something
' >> README.md

echo '
Copyright <YEAR> <COPYRIGHT HOLDER>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CO
' >> LICENSE.md

mkdir src
cd src
mkdir server
mkdir client
mkdir characters
mkdir modules
cd server
echo 'print("Hello from the server!")
' >> sr.server.lua
cd ..
cd client
echo 'print("Hello from the client!")
' >> cl.client.lua

echo -e "${GREEN}Proccess finished!"