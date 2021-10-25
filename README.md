# Goals
- Do code mods without touching the business logic
- Do code transforms programmatically if possible

# Refactor Steps (done in order)
- Remove top level directories that are not a part of source code
- Move everything that is js or jsx to one file, and all .scss into one file, with the "src/" path at "top" of every file for reference
- Move translations to top level src directory
- Remove all: mocks, tests, .stories (storybook) (mostly can be done by skipping files when reading)
- Remove all imports that are not npm dependencies (they should be available in the file scope)
- Remove all export statements
- Remove all commented code blocks
- Remove some JSDoc comments if they are verbose or not helpful and/or replaced with TypeScript
- Install Parcel or Snowpack (near instant reload vs webpack)
- Make the code compile - fix runtime errors
- Move to TypeScript
- Remove all dependencies that are unused from package.json
- Update all dependencies (to latest major versions, including breaking changes)
- Replace scss with tailwind css
- Begin work on removing redux and rxjs - moving to a single context store as well as local state.
- Move class components to hooks
- Remove all *dead* code (TS will really show this!)
- Remove *duplicate* code (as much as realistically possible)
- All reusable non-react code and associated types moved to a single file "lib.ts"
- All reusable react code and associated types moved to single file "libReact.tsx"
- Internationalization is incomplete (see `validateFormBuilderForm`) therefore it should either be taken seriously ASAP or removed
- Find out how to have a single error / loading / success type instead of hundreds of them (however success must be unique)
- Move all high-level constants and types to top of file, constants in all caps with underscores
- Make use of TypeScript enumerations
- Convert all PropTypes to TypeScript types
- Use a single UUID generation function (the "uuid" npm package) - do a search for "Math.random()" to find where this could be done
- Use a standard font cascade that makes use of native fonts
- Reduce client side logging and monitoring if not being used
- Remove dynamic webpack imports - bundle size will be minimal after refactor (under ~3mb vs current 15mb)
- Move away from FrontAwesome (if it is paid?) and use something like feather icons or similar (free)
- Remove all console.log() statements
- Remove script injection using `.innerHTML`


# Infrastructure and Configuration
- Move all URLs to a .env file and use the `dotenv` package to load them
- Use a lightweight node:latest docker container for building and running locally
- Minimize package.json script usage and prefer docker + shell
- Use a faster package manager such as pnpm
- Use nginx or similar proxy for building and running in production (faster, more secure than express)

# Standards
- Avoid regex and string-type programming
- Only store config externally
- Do not create new functions and components unless they are being reused. "Breaking things down" does not help, unless it is for reuse.
- Avoid null and undefined

# Timeline and distribution of work
- 3 weeks of development freeze in December

# General Ideas
Use render props for reusable query component? (to avoid many duplicate error/loading states)
useReducer() for more complex local state instead of many useState()
Centralize and deduplicate networking code
Do easy stuff first and follow IDE/TS hints
