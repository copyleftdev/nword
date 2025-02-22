# Makefile for nword

# Variables
BINARY_NAME = nword
CARGO = cargo
INSTALL_DIR = ${HOME}/.local/bin

# Default target
all: build

# Build the project in release mode
build:
	@echo "Building $(BINARY_NAME)..."
	@$(CARGO) build --release
	@echo "Build complete!"

# Run tests
test:
	@echo "Running tests..."
	@$(CARGO) test
	@echo "Tests complete!"

# Install the binary to the user's local bin directory
install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@mkdir -p $(INSTALL_DIR)
	@cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installation complete! $(BINARY_NAME) is now available in $(INSTALL_DIR)."

# Uninstall the binary
uninstall:
	@echo "Uninstalling $(BINARY_NAME)..."
	@rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Uninstallation complete!"

# Clean the project (remove build artifacts)
clean:
	@echo "Cleaning build artifacts..."
	@$(CARGO) clean
	@echo "Clean complete!"

# Run the project locally
run:
	@echo "Running $(BINARY_NAME)..."
	@$(CARGO) run -- --directory .

# Help target
help:
	@echo "Available targets:"
	@echo "  build    - Build the project in release mode"
	@echo "  test     - Run tests"
	@echo "  install  - Install the binary to $(INSTALL_DIR)"
	@echo "  uninstall - Uninstall the binary"
	@echo "  clean    - Remove build artifacts"
	@echo "  run      - Run the project locally"
	@echo "  help     - Show this help message"

# Default target
.DEFAULT_GOAL := help