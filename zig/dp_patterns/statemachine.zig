/// DP State machine pattern -> https://blog.algomaster.io/p/20-patterns-to-master-dynamic-programming
const std = @import("std");

fn best_time_to_buy_and_sell_stocks(prices: []const i32) i32 {
    var profit: i32, var loss: i32 = .{0, -prices[0]};
    for (1..prices.len) |i| {
        const prev_profit: i32, const prev_loss: i32 = .{profit, loss};
        loss = @max(prev_loss, -prices[i]);
        profit = @max(prev_profit, prev_loss + prices[i]);
    }
    return profit;
}

test "lc_121" {
    const testcases = [_]struct {
        prices: []const i32,
        expected: i32
    } {
        .{
            .prices = &.{7, 1, 5, 3, 6, 4},
            .expected = 5
        }, 
        .{
            .prices = &.{7, 6, 4, 3, 1},
            .expected = 0
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, best_time_to_buy_and_sell_stocks(tc.prices));
    }
}

fn best_time_to_buy_and_sell_stocks_ii(prices: []const i32) i32{
    var profit: i32, var loss: i32 = .{0, -prices[0]};
    for (1..prices.len) |i| {
        const prev_profit: i32, const prev_loss: i32 = .{profit, loss};
        loss = @max(prev_loss, prev_profit - prices[i]);
        profit = @max(prev_profit, prev_loss + prices[i]);
    }
    return profit;
}

test "lc_122" {
    const testcases = [_]struct {
        prices: []const i32,
        expected: i32
    } {
        .{
            .prices = &.{7, 1, 5, 3, 6, 4},
            .expected = 7
        }, 
        .{
            .prices = &.{7, 6, 4, 3, 1},
            .expected = 0
        },
        .{
            .prices = &.{1, 2, 3, 4 ,5},
            .expected = 4
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, best_time_to_buy_and_sell_stocks_ii(tc.prices));
    }
}

fn best_time_to_buy_and_sell_stocks_iii(prices: []const i32) i32 {
    var p1: i32, var l1: i32, var p2: i32, var l2: i32 = .{0, -prices[0], 0, std.math.minInt(i32)};
    for (1..prices.len) |i| {
        const prev_p1: i32, const prev_l1: i32, const prev_p2: i32, const prev_l2:i32 = .{p1, l1, p2, l2};
            l1 = @max(prev_l1, -prices[i]);
            p1 = @max(prev_p1, prev_l1 + prices[i]);

            l2 = @max(prev_l2, prev_l1 - prices[i]);
            p2 = @max(prev_p2, prev_l2 + prices[i]);
    }
    return @max(p1, p2);
}


test "lc_123" {
    const testcases = [_]struct {
        prices: []const i32,
        expected: i32
    } {
        .{
            .prices = &.{3, 3, 5, 0, 0, 3, 1, 4},
            .expected = 4
        }, 
        .{
            .prices = &.{7, 6, 4, 3, 1},
            .expected = 0
        },
        .{
            .prices = &.{1, 2, 3, 4 ,5},
            .expected = 4
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, best_time_to_buy_and_sell_stocks_iii(tc.prices));
    }
}

fn best_time_to_buy_and_sell_stocks_iv(gpa: std.mem.Allocator, prices: []const i32, k: i32) i32 {
    if (@as(usize, @intCast(2*k)) > prices.len) {
        var sum: i32 = 0;
        for (1..prices.len) |i| {
            if (prices[i] > prices[i - 1]) { sum += prices[i] - prices[i - 1]; }
        }
        return sum;
    }

    var dp = gpa.alloc(i32, @intCast(2*k+1)) catch unreachable;
    defer gpa.free(dp); 

    for(1..@intCast(2*k+1)) |i| {
        if (i % 2 == 0) { dp[i] = std.math.minInt(i32); } 
        else { dp[i] = 0; }
    }

    for (prices) |price| {
        for (0..@intCast(2*k+1)) |i| {
            if (i == 0) { dp[i] = @max(dp[i], -price); }
            else if (i % 2 == 0) { dp[i] = @max(dp[i], dp[i - 1] - price); }
            else { dp[i] = @max(dp[i], dp[i - 1] + price); }
        }
    }
    return dp[@intCast(2*k-1)];
}

test "lc_188" {
    const testcases = [_]struct {
        prices: []const i32,
        k: i32,
        expected: i32
    } {
        .{
            .prices = &.{3, 2, 6, 5, 0, 3},
            .k = 2,
            .expected = 7
        }, 
        .{
            .prices = &.{2, 4, 1},
            .k = 2, 
            .expected = 2
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, best_time_to_buy_and_sell_stocks_iv(std.testing.allocator, tc.prices, tc.k));
    }
}

// fn best_time_to_buy_and_sell_stock_with_transaction_fee(prices: []const i32, fee: i32) i32 {
//
// }
//
// test "lc_714" {
//
// }
