/*
 * Copyright (c) 2006-Present, Redis Ltd.
 * All rights reserved.
 *
 * Licensed under your choice of the Redis Source Available License 2.0
 * (RSALv2) or the Server Side Public License v1 (SSPLv1), or the GNU Affero
 * General Public License v3 (AGPLv3), as the case may be.
 */

/*
 * Comprehensive tests for logging functionality
 * Tests variadic arguments (0-5 args), edge cases, and safety
 */

#include "test_util.h"
#include "util/logging.h"
#include "rmutil/alloc.h"
#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <float.h>
#include <stdarg.h>

// Forward declarations
typedef struct RedisModuleCtx RedisModuleCtx;
extern void (*RedisModule_Log)(RedisModuleCtx *, const char *, const char *, ...);

// Mock Redis logging - capture the last logged message
static char lastLogLevel[32] = {0};
static char lastLogMessage[2048] = {0};
static int logCallCount = 0;

static void mockRedisLog(RedisModuleCtx *ctx, const char *level, const char *fmt, ...) {
    logCallCount++;
    strncpy(lastLogLevel, level, sizeof(lastLogLevel) - 1);

    va_list ap;
    va_start(ap, fmt);
    vsnprintf(lastLogMessage, sizeof(lastLogMessage), fmt, ap);
    va_end(ap);
}

// Helper to reset log state between tests
static void resetLogState() {
    memset(lastLogLevel, 0, sizeof(lastLogLevel));
    memset(lastLogMessage, 0, sizeof(lastLogMessage));
    logCallCount = 0;
}

// ============================================================================
// Test 0 Arguments - Simple strings, no format specifiers
// ============================================================================

int testLogCallback_0Args() {
    resetLogState();

    // Simple messages with no arguments
    LogCallback("debug", "Server starting");
    ASSERT_STRING_EQ(lastLogLevel, "debug");
    ASSERT_STRING_EQ(lastLogMessage, "Server starting");

    LogCallback("verbose", "Loading configuration");
    ASSERT_STRING_EQ(lastLogLevel, "verbose");
    ASSERT_STRING_EQ(lastLogMessage, "Loading configuration");

    LogCallback("notice", "Initialization complete");
    ASSERT_STRING_EQ(lastLogLevel, "notice");
    ASSERT_STRING_EQ(lastLogMessage, "Initialization complete");

    LogCallback("warning", "Cache nearly full");
    ASSERT_STRING_EQ(lastLogLevel, "warning");
    ASSERT_STRING_EQ(lastLogMessage, "Cache nearly full");

    ASSERT_EQUAL(4, logCallCount);
    return 0;
}

// ============================================================================
// Test 1 Argument - Single format specifier with various types
// ============================================================================

int testLogCallback_1Arg() {
    resetLogState();

    // String argument
    const char *index_name = "my_index";
    LogCallback("debug", "Processing index: %s", index_name);
    ASSERT_STRING_EQ(lastLogMessage, "Processing index: my_index");

    // Integer argument
    int doc_count = 42;
    LogCallback("notice", "Document count: %d", doc_count);
    ASSERT_STRING_EQ(lastLogMessage, "Document count: 42");

    // Float argument
    double query_time = 15.67;
    LogCallback("verbose", "Query time: %.2fms", query_time);
    ASSERT_STRING_EQ(lastLogMessage, "Query time: 15.67ms");

    // Unsigned argument
    unsigned int term_count = 1000;
    LogCallback("debug", "Term count: %u", term_count);
    ASSERT_STRING_EQ(lastLogMessage, "Term count: 1000");

    // Long long argument
    long long large_value = 9223372036854775807LL;
    LogCallback("debug", "Large value: %lld", large_value);
    ASSERT_STRING_EQ(lastLogMessage, "Large value: 9223372036854775807");

    ASSERT_EQUAL(5, logCallCount);
    return 0;
}

// ============================================================================
// Test 2 Arguments
// ============================================================================

int testLogCallback_2Args() {
    resetLogState();

    const char *index_name = "products";
    int doc_count = 1000;

    // String + Integer
    LogCallback("notice", "Index %s has %d documents", index_name, doc_count);
    ASSERT_STRING_EQ(lastLogMessage, "Index products has 1000 documents");

    // Integer + Float
    double avg_time = 12.34;
    LogCallback("verbose", "Processed %d queries in %.2fms avg", doc_count, avg_time);
    ASSERT_STRING_EQ(lastLogMessage, "Processed 1000 queries in 12.34ms avg");

    // String + String
    const char *field_name = "title";
    LogCallback("debug", "Indexing field %s in index %s", field_name, index_name);
    ASSERT_STRING_EQ(lastLogMessage, "Indexing field title in index products");

    ASSERT_EQUAL(3, logCallCount);
    return 0;
}

// ============================================================================
// Test 3 Arguments
// ============================================================================

int testLogCallback_3Args() {
    resetLogState();

    const char *index_name = "products";
    int doc_count = 1000;
    double query_time = 15.67;

    LogCallback("notice", "Index %s: %d documents, %.2fms query time",
                index_name, doc_count, query_time);
    ASSERT(strstr(lastLogMessage, "products") != NULL);
    ASSERT(strstr(lastLogMessage, "1000") != NULL);
    ASSERT(strstr(lastLogMessage, "15.67") != NULL);

    const char *query = "search term";
    int results = 42;
    LogCallback("verbose", "Query '%s' found %d results in index %s",
                query, results, index_name);
    ASSERT(strstr(lastLogMessage, "search term") != NULL);
    ASSERT(strstr(lastLogMessage, "42") != NULL);

    return 0;
}

// ============================================================================
// Test 4 Arguments
// ============================================================================

int testLogCallback_4Args() {
    resetLogState();

    const char *index_name = "products";
    int doc_count = 1000;
    double query_time = 15.67;
    int cache_hits = 150;

    LogCallback("verbose", "Index %s: %d docs, %.2fms query, %d cache hits",
                index_name, doc_count, query_time, cache_hits);
    ASSERT(strstr(lastLogMessage, "products") != NULL);
    ASSERT(strstr(lastLogMessage, "150") != NULL);

    const char *operation = "INSERT";
    int affected_rows = 5;
    double duration = 3.14;
    LogCallback("debug", "Operation %s affected %d rows in %.2fms on index %s",
                operation, affected_rows, duration, index_name);
    ASSERT(strstr(lastLogMessage, "INSERT") != NULL);

    return 0;
}

// ============================================================================
// Test 5 Arguments
// ============================================================================

int testLogCallback_5Args() {
    resetLogState();

    const char *index_name = "products";
    int doc_count = 1000;
    double query_time = 15.67;
    int cache_hits = 150;
    double cache_ratio = 0.85;

    LogCallback("verbose",
                "Index %s: %d docs, %.2fms query, %d cache hits, %.1f%% ratio",
                index_name, doc_count, query_time, cache_hits, cache_ratio * 100);
    ASSERT(strstr(lastLogMessage, "products") != NULL);
    ASSERT(strstr(lastLogMessage, "85.0%") != NULL);

    const char *user = "admin";
    const char *action = "DELETE";
    int count = 10;
    const char *target = "expired_docs";
    double elapsed = 2.5;
    LogCallback("notice",
                "User %s performed %s on %d %s records in %.2fms",
                user, action, count, target, elapsed);
    ASSERT(strstr(lastLogMessage, "admin") != NULL);
    ASSERT(strstr(lastLogMessage, "DELETE") != NULL);

    return 0;
}

// ============================================================================
// Test Long Messages - Truncation behavior
// ============================================================================

int testLogCallback_LongMessages() {
    resetLogState();

    // Test message exactly at limit
    char exact_msg[1024];
    memset(exact_msg, 'A', 1023);
    exact_msg[1023] = '\0';
    LogCallback("debug", "%s", exact_msg);
    
    // Test message exceeding limit (should truncate)
    char long_msg[2048];
    memset(long_msg, 'B', 2047);
    long_msg[2047] = '\0';
    LogCallback("warning", "%s", long_msg);
    
    // Test formatted message that becomes too long
    const char *repeated = "This is a repeating pattern. ";
    LogCallback("verbose", "%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s",
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated,
                repeated, repeated, repeated, repeated, repeated);
    
    return 0;
}

// ============================================================================
// Test Special Characters
// ============================================================================

int testLogCallback_SpecialCharacters() {
    resetLogState();

    // Percent signs (should not be interpreted as format specifiers)
    LogCallback("notice", "Progress: 100%% complete");
    
    // Newlines
    LogCallback("debug", "Line 1\nLine 2\nLine 3");
    
    // Tabs
    LogCallback("verbose", "Column1\tColumn2\tColumn3");
    
    // Quotes
    LogCallback("notice", "Index \"products\" created");
    LogCallback("debug", "Query: 'SELECT * FROM users'");
    
    // Mixed special characters
    const char *json = "{\"name\": \"test\", \"value\": 42}";
    LogCallback("verbose", "JSON data: %s", json);
    
    return 0;
}

// ============================================================================
// Test Edge Cases
// ============================================================================

int testLogCallback_EdgeCases() {
    resetLogState();

    // Empty string
    LogCallback("debug", "");
    
    // Just a format specifier with empty string
    LogCallback("verbose", "%s", "");
    
    // Zero values
    LogCallback("notice", "Zero int: %d", 0);
    LogCallback("debug", "Zero float: %.2f", 0.0);
    
    // NULL-like patterns (not actual NULL - that would be UB)
    LogCallback("verbose", "Empty result: %s", "(null)");
    
    return 0;
}

// ============================================================================
// Test Numeric Boundaries
// ============================================================================

int testLogCallback_NumericBoundaries() {
    resetLogState();

    // Integer limits
    LogCallback("debug", "INT_MAX: %d", INT_MAX);
    LogCallback("debug", "INT_MIN: %d", INT_MIN);
    
    // Unsigned max
    LogCallback("verbose", "UINT_MAX: %u", UINT_MAX);
    
    // Long long limits
    LogCallback("debug", "LLONG_MAX: %lld", LLONG_MAX);
    LogCallback("debug", "LLONG_MIN: %lld", LLONG_MIN);
    
    // Float extremes
    LogCallback("verbose", "Large float: %.2f", 999999999.99);
    LogCallback("verbose", "Small float: %.10f", 0.0000000001);
    
    // Negative values
    LogCallback("debug", "Negative: %d", -42);
    LogCallback("verbose", "Negative float: %.2f", -123.45);
    
    return 0;
}

// ============================================================================
// Test Format String Variations
// ============================================================================

int testLogCallback_FormatVariations() {
    resetLogState();

    int value = 255;
    
    // Hexadecimal
    LogCallback("debug", "Hex: 0x%x", value);
    LogCallback("verbose", "Hex (uppercase): 0x%X", value);
    
    // Octal
    LogCallback("debug", "Octal: %o", value);
    
    // Padding
    LogCallback("verbose", "Padded: %05d", 42);
    LogCallback("debug", "Left-aligned: %-10s", "test");
    
    // Precision
    double pi = 3.14159265359;
    LogCallback("notice", "Pi (2 decimals): %.2f", pi);
    LogCallback("verbose", "Pi (5 decimals): %.5f", pi);
    
    // Width and precision
    LogCallback("debug", "Formatted: %10.2f", 123.456);
    
    return 0;
}

// ============================================================================
// Test All Log Levels
// ============================================================================

int testLogCallback_AllLogLevels() {
    resetLogState();

    const char *msg = "Test message";
    
    LogCallback("debug", "Debug: %s", msg);
    LogCallback("verbose", "Verbose: %s", msg);
    LogCallback("notice", "Notice: %s", msg);
    LogCallback("warning", "Warning: %s", msg);
    
    return 0;
}

// ============================================================================
// Test Real-World Patterns (typical RediSearch usage)
// ============================================================================

int testLogCallback_RealWorldPatterns() {
    resetLogState();

    // Index creation
    LogCallback("notice", "Creating index '%s' with %d fields", "products", 10);
    
    // Document indexing
    LogCallback("verbose", "Indexed document %lld in %.2fms", 123456789LL, 1.23);
    
    // Query execution
    LogCallback("debug", "Executing query: %s", "(@title:laptop @price:[100 500])");
    LogCallback("verbose", "Query returned %d results in %.2fms", 42, 15.67);
    
    // Cache statistics
    LogCallback("debug", "Cache stats: %d hits, %d misses, %.1f%% hit rate", 
                150, 50, 75.0);
    
    // Error conditions
    LogCallback("warning", "Index '%s' memory usage: %d MB (threshold: %d MB)",
                "large_index", 950, 1000);
    
    // Performance metrics
    LogCallback("verbose", "Index '%s': %d docs, %u terms, %.2f MB, avg doc size: %d bytes",
                "products", 1000000, 5000000, 512.5, 512);
    
    return 0;
}

// ============================================================================
// Test Suite Registration
// ============================================================================

int main(int argc, char **argv) {
    printf("Starting Test '%s'...\n", argv[0]);

    // Initialize allocator and mock Redis logging
    RMUTil_InitAlloc();
    RedisModule_Log = mockRedisLog;

    // Run all tests
    TESTFUNC(testLogCallback_0Args);
    TESTFUNC(testLogCallback_1Arg);
    TESTFUNC(testLogCallback_2Args);
    TESTFUNC(testLogCallback_3Args);
    TESTFUNC(testLogCallback_4Args);
    TESTFUNC(testLogCallback_5Args);
    TESTFUNC(testLogCallback_LongMessages);
    TESTFUNC(testLogCallback_SpecialCharacters);
    TESTFUNC(testLogCallback_EdgeCases);
    TESTFUNC(testLogCallback_NumericBoundaries);
    TESTFUNC(testLogCallback_FormatVariations);
    TESTFUNC(testLogCallback_AllLogLevels);
    TESTFUNC(testLogCallback_RealWorldPatterns);

    PRINT_TEST_SUMMARY;
    printf("\n--------------------\n\n");
    return 0;
}