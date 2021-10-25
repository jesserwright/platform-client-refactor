lines in index.jsx: 165399 - 155796

# Goals
- Improve source code clarity, correctness, performance, robustness, modifiability, and security
- Reduce dependencies, and duplication.
- Reduce Lines Of Code (LOC) by 50% (10% styled components, 10% function duplication, 10% mocks/tests, 10% SCSS, 10% barrel files / "commented out" code blocks / dead code)

# Process methods
- Do code mods without touching the business logic
- Do code transforms programmatically if possible
- Make use of tools to refactor automatically when possible

# Reorganize source, delete unneeded code and documentation
- Remove top level directories that are not a part of source code
- Move everything that is js or jsx to one file, and all .scss into one file, with the "src/" path at "top" of every file for reference
- Move translations to top level src directory
- Remove all: mocks, tests, .stories (storybook), cypress tests (mostly can be done by skipping files when reading)
- Remove all import and export statements that are not npm dependencies (they should be available in the file scope, since the whole FE is one file)

# Compile on new stack
- Install to Parcel bundler, pnpm
- Make the code compile - fix runtime errors

# Code quality improvements
- Remove all commented out code blocks
- Remove some JSDoc comments if they are verbose or not helpful
- Remove all dependencies that are unused from package.json
- Update all dependencies (to latest major versions, including breaking changes)
- Remove script injection using `.innerHTML` and find appropriate package(s) on npm
- Remove all console.log() statements

# New libraries
- Tailwind CSS
- TypeScript
- UUID (proper random ID generation)

# Libraries to remove
- Styled components
- Redux (use React.Context)
- RxJS (use React.Context)
- PropTypes (use TypeScript)

- Move class components to hooks
- Remove all *dead* code (TS will really show this!)
- Remove *duplicate* code (as much as realistically possible)
- All reusable non-react code and associated types moved to a single file "lib.ts"
- All reusable react code and associated component-specific types moved to single file "libReact.tsx"

- Internationalization is incomplete (see `validateFormBuilderForm`) therefore it should either be taken seriously ASAP or removed
- Find out how to have a single error / loading / success type instead of hundreds of them (however success must be unique)
- Move all high-level constants and types to top of file, constants in all caps with underscores
- Use a standard font cascade that makes use of native fonts
- Reduce client side logging and monitoring if not being used
- Remove dynamic webpack imports - bundle size will be minimal after refactor (under ~3mb vs current 15mb)
- Move away from FrontAwesome (if it is paid and requires special npm source) and use something like feather icons or similar

# Infrastructure and Configuration (roughly in order)
- Use a lightweight NodeJS Docker container for building and developing locally
- Move all configuration URLs (currently embedded in source code) to .env files and use the `dotenv` package to load them. .env.prod and .env.dev may be used for their respective intents.
- Minimize package.json script usage and prefer docker + shell
- Use nginx or similar proxy for building and running in production (faster, more secure than express)
- Merge `platform-client` into the `mono-platform` repo so Front End and Back End code co-exist

# Programming Standards
- Avoid regex and string-type programming
- Only store config externally
- Avoid creating new functions and components unless they are being reused. "Breaking things down" does not help, unless it is for reuse.
- Avoid null and undefined

# Timeline and distribution of work
- 3 weeks of development freeze in December

# Idea, Questions
- Use render props for reusable query component? (to avoid many duplicate error/loading states)
- useReducer() for more complex local state instead of many useState() calls
- Centralize and deduplicate networking code
