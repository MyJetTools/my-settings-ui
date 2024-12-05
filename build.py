import random
import string
import argparse

def replace_wasm_with_random_string(file_path):
    # Generate a random string of length 16
    def generate_random_string(length=16):
        characters = string.ascii_letters + string.digits  # A-Z, a-z, 0-9
        return ''.join(random.choice(characters) for _ in range(length))


    print(file_path)
    # Read the file
    with open(file_path, 'r') as file:
        content = file.read()

    # Replace occurrences of ".wasm" with ".wasm?id=RANDOM_STRING"
    updated_content = content.replace('.wasm', f'.wasm?id={generate_random_string()}')

    updated_content = updated_content.replace('.js', f'.js?id={generate_random_string()}')

    updated_content = updated_content.replace('.css', f'.css?id={generate_random_string()}')

    print(updated_content)

    # Write the updated content back to the file
    with open(file_path, 'w') as file:
        file.write(updated_content)

    #print(f"Updated file {file_path} successfully.")

if __name__ == "__main__":
    # Set up argument parser
    parser = argparse.ArgumentParser(description="Replace .wasm with .wasm?id=RANDOM_STRING")
    parser.add_argument("file_path", help="Path to the file you want to modify")
    
    # Parse command-line arguments
    args = parser.parse_args()
    
    # Call the function with the file path argument
    replace_wasm_with_random_string(args.file_path)