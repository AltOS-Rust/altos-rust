#! /bin/sh

# recursively traverse repo structure
# on the lookout for rogue files without headers
# if we see them, we add a header structure to the file

# if the file already has a file header, we ignore it, because 
# we are #awesomelikethat

# looking only for .rs files at this point

root_dir=$(pwd)
header_file=$(cat $root_dir/header.txt)

function main(){
  # get all the .rs filename full paths
  file_list=$(find $root_dir -name *.rs)
  # loop through the list of files
  for file in $file_list; do
    # check if the header already exists
    # if so, dont replace it, leave the file alone
    # if it doesnt exist we need to create it
    head -15 $file | grep "GNU"
    status=$?
    if [ $status -eq 0 ]; then
      # header already exists
      echo "File $file already has header."
      # go to next file
      continue
    fi
    # else header does not exist
    echo "Adding header to $file"
    echo "$header_file\n\n$(cat $file)" > $file
  done
}

# start the script
main
