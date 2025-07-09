using Microsoft.AspNetCore.SignalR;
using wavewatch_server;

var builder = WebApplication.CreateBuilder(args);

// Load CORS origins from config
var allowedOrigins = builder.Configuration.GetSection("AllowedOrigins").Get<string[]>();

builder.Services.AddCors(options =>
{
    options.AddPolicy("AllowFrontend", policy =>
    {
        policy.WithOrigins(allowedOrigins!)
            .AllowAnyHeader()
            .AllowAnyMethod()
            .AllowCredentials();
    });
});
builder.Services.AddOpenApi();
builder.Services.AddSignalR();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseCors("AllowFrontend");

app.MapPost("/analyzers/{analyzerId}/traces", async (string analyzerId, TraceData data, IHubContext<AnalyzerHub, IAnalyzerClient> hubContext) =>
{
    // Pass along TraceData to all clients part of the group associated with the 'analyzerId'
    await hubContext.Clients.Group(analyzerId).ReceiveTraceData(analyzerId, data);
    return Results.Ok("Trace uploaded and broadcasted");
});
app.MapHub<AnalyzerHub>("/analyzerhub");

app.Run();

