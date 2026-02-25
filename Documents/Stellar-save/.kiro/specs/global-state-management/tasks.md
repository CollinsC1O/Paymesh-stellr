# Implementation Plan: Global State Management

## Overview

This implementation plan breaks down the global state management feature into actionable tasks. The implementation uses Zustand with immer middleware for immutable updates and persistence middleware for localStorage/sessionStorage integration. The tasks follow an incremental approach: set up dependencies and structure, implement core slices, add persistence, create hooks and utilities, write tests, and finally integrate with the existing application.

## Tasks

- [ ] 1. Install dependencies and set up project structure
  - Install zustand, immer, and @fast-check/vitest packages
  - Create store directory structure (store/, store/slices/, store/middleware/, store/__tests__/)
  - Create placeholder files for type definitions and test utilities
  - _Requirements: 1.1, 1.2, 1.3_

- [ ] 2. Define TypeScript types and interfaces
  - [ ] 2.1 Create core type definitions in store/types.ts
    - Define ViewMode, Theme, CurrencyDisplay types
    - Define UIState and PreferencesState interfaces
    - Define UIActions and PreferencesActions interfaces
    - Define AppStoreState and AppStore combined types
    - Define PersistenceConfig interface
    - _Requirements: 2.4, 2.5, 9.1, 9.2, 9.3_
  
  - [ ] 2.2 Define default state constants
    - Create DEFAULT_UI_STATE constant with initial values
    - Create DEFAULT_PREFERENCES constant with initial values
    - Export constants for use in store creation
    - _Requirements: 2.1, 3.1_

- [ ] 3. Implement UI state slice
  - [ ] 3.1 Create uiSlice.ts with state and actions
    - Implement toggleSidebar action
    - Implement openModal and closeModal actions
    - Implement setViewMode action
    - Implement setTheme action
    - Use immer for immutable updates
    - _Requirements: 2.2, 2.5, 2.6, 3.2, 3.4_
  
  - [ ]* 3.2 Write unit tests for UI slice actions
    - Test toggleSidebar toggles between true/false
    - Test openModal sets activeModal correctly
    - Test closeModal sets activeModal to null
    - Test setViewMode updates viewMode
    - Test setTheme updates theme
    - _Requirements: 3.5_

- [ ] 4. Implement user preferences slice
  - [ ] 4.1 Create preferencesSlice.ts with state and actions
    - Implement setLanguage action
    - Implement setCurrencyDisplay action
    - Implement toggleNotifications action
    - Implement toggleSound action
    - Implement resetPreferences action
    - Use immer for immutable updates
    - _Requirements: 2.3, 2.5, 2.6, 3.2, 3.4_
  
  - [ ]* 4.2 Write unit tests for preferences slice actions
    - Test setLanguage updates language
    - Test setCurrencyDisplay updates currency display
    - Test toggleNotifications toggles notifications
    - Test toggleSound toggles sound
    - Test resetPreferences restores defaults
    - _Requirements: 3.5_

- [ ] 5. Implement persistence middleware
  - [ ] 5.1 Create persistence configuration in store/middleware/persistence.ts
    - Configure UI state persistence to sessionStorage
    - Configure preferences persistence to localStorage
    - Implement error handling for corrupted data
    - Implement error handling for storage quota exceeded
    - Implement error handling for unavailable storage
    - Add version support for schema migrations
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6_
  
  - [ ]* 5.2 Write unit tests for persistence behavior
    - Test preferences save to localStorage on update
    - Test UI state saves to sessionStorage on update
    - Test store hydrates from storage on initialization
    - Test corrupted storage data falls back to defaults
    - Test missing storage fields merge with defaults
    - Test storage quota exceeded handled gracefully
    - Test storage unavailable handled gracefully
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ] 6. Create combined Zustand store
  - [ ] 6.1 Implement store creation in store/index.ts
    - Combine UI slice and preferences slice
    - Apply immer middleware for immutable updates
    - Apply persistence middleware for both slices
    - Configure Redux DevTools integration (development only)
    - Export store instance and types
    - _Requirements: 1.4, 2.1, 5.1, 5.2_
  
  - [ ]* 6.2 Write unit tests for store creation and initialization
    - Test store initializes with correct default values
    - Test store structure includes all required slices
    - Test DevTools enabled in development environment
    - Test DevTools disabled in production environment
    - _Requirements: 2.1, 5.1, 5.2_

- [ ] 7. Implement custom hooks for state access
  - [ ] 7.1 Create custom hooks in store/index.ts
    - Implement useUIState hook with optional selector
    - Implement usePreferences hook with optional selector
    - Implement useAppStore hook with optional selector
    - Add TypeScript overloads for selector variants
    - _Requirements: 6.4, 6.5, 7.2, 9.2, 9.3_
  
  - [ ]* 7.2 Write integration tests for React hooks
    - Test components using useUIState re-render on UI state changes
    - Test components using usePreferences re-render on preference changes
    - Test components don't re-render on unrelated state changes
    - Test multiple components can subscribe to same state
    - Test hooks work with React Testing Library
    - _Requirements: 6.5, 7.1, 7.2_

- [ ] 8. Create testing utilities
  - [ ] 8.1 Implement test utilities in store/testUtils.ts
    - Create createTestStore function for isolated store instances
    - Support custom initial state for test scenarios
    - Ensure compatibility with React Testing Library
    - Add utilities for spying on actions
    - Add utilities for mocking state
    - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5_
  
  - [ ]* 8.2 Write unit tests for testing utilities
    - Test createTestStore creates isolated instances
    - Test test store accepts custom initial state
    - Test actions can be spied on in tests
    - Test state can be mocked for component tests
    - _Requirements: 10.1, 10.2, 10.4, 10.5_

- [ ] 9. Checkpoint - Ensure all unit tests pass
  - Run all unit tests and verify they pass
  - Ensure all tests pass, ask the user if questions arise

- [ ]* 10. Write property-based tests for correctness properties
  - [ ]* 10.1 Write property test for immutable state updates
    - **Property 1: Immutable State Updates**
    - **Validates: Requirements 2.6, 3.4**
    - Generate random initial states and actions
    - Verify previous state object remains unchanged after action
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.2 Write property test for preferences persistence
    - **Property 2: Preferences Persistence to localStorage**
    - **Validates: Requirements 4.1**
    - Generate random preference values
    - Verify localStorage contains updated values after action
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.3 Write property test for UI state persistence
    - **Property 3: UI State Persistence to sessionStorage**
    - **Validates: Requirements 4.2**
    - Generate random UI state values
    - Verify sessionStorage contains updated values after action
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.4 Write property test for serialization format
    - **Property 4: Persistence Serialization Format**
    - **Validates: Requirements 4.5**
    - Generate random states
    - Verify stored values are valid JSON that can be parsed
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.5 Write property test for persistence round-trip
    - **Property 5: Persistence Round-Trip**
    - **Validates: Requirements 4.7**
    - Generate random valid store states
    - Verify persist→hydrate preserves all values
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.6 Write property test for selective re-rendering
    - **Property 6: Selective Re-rendering**
    - **Validates: Requirements 6.5, 7.1**
    - Generate random state changes in different slices
    - Verify components only re-render when their selected state changes
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.7 Write property test for selector memoization
    - **Property 7: Selector Memoization**
    - **Validates: Requirements 7.3**
    - Generate random states and expensive selectors
    - Verify selectors return memoized results without recomputing
    - Use @fast-check/vitest with 100+ iterations
  
  - [ ]* 10.8 Write property test for batched state updates
    - **Property 8: Batched State Updates**
    - **Validates: Requirements 7.4**
    - Generate random sequences of synchronous state updates
    - Verify only one component re-render occurs per batch
    - Use @fast-check/vitest with 100+ iterations

- [ ] 11. Integrate store with existing application
  - [ ] 11.1 Update main.tsx to initialize store
    - Import store at application entry point
    - Ensure store initialization doesn't block WalletProvider
    - Verify no provider wrapping required for Zustand
    - _Requirements: 6.1, 6.2, 6.3, 8.1, 8.4_
  
  - [ ] 11.2 Create example component using new state management
    - Create a simple component that uses useUIState
    - Create a simple component that uses usePreferences
    - Demonstrate coexistence with useWallet hook
    - Add to application to verify integration
    - _Requirements: 6.4, 6.5, 8.2, 8.3, 8.5_
  
  - [ ]* 11.3 Write integration test for WalletProvider coexistence
    - Test store works alongside WalletProvider without conflicts
    - Test both useWallet and useUIState work in same component
    - Test store initialization doesn't block WalletProvider initialization
    - _Requirements: 1.5, 8.1, 8.2, 8.3, 8.4, 8.5_

- [ ] 12. Configure DevTools for development
  - [ ] 12.1 Set up Redux DevTools integration
    - Configure DevTools to display current state values
    - Configure DevTools to display state change history
    - Configure DevTools to log action names and payloads
    - Ensure DevTools only enabled in development environment
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_
  
  - [ ]* 12.2 Write unit tests for DevTools configuration
    - Test DevTools enabled in development
    - Test DevTools disabled in production
    - Test DevTools connection failure doesn't break store
    - _Requirements: 5.1, 5.2_

- [ ] 13. Final checkpoint - Verify complete integration
  - Run all tests (unit and property-based) and verify they pass
  - Test the application manually to ensure state management works end-to-end
  - Verify persistence works across browser sessions
  - Verify DevTools integration in development mode
  - Ensure all tests pass, ask the user if questions arise

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP delivery
- Each task references specific requirements for traceability
- Property-based tests validate universal correctness properties across all inputs
- Unit tests validate specific examples, edge cases, and error conditions
- The implementation uses TypeScript throughout for type safety
- Zustand requires no provider wrapping, simplifying integration with existing WalletProvider
- Persistence is handled via middleware, keeping slice code clean and focused
- DevTools integration provides powerful debugging capabilities in development
