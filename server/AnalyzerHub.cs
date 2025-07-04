using Microsoft.AspNetCore.SignalR;

namespace wavewatch_server;

public class AnalyzerHub : Hub<IAnalyzerClient>
{
    public Task ConnectToAnalyzer(string analyzerId)
    {
        return Groups.AddToGroupAsync(Context.ConnectionId, analyzerId);        
    }

    public Task DisconnectFromAnalyzer(string analyzerId)
    {
        return Groups.RemoveFromGroupAsync(Context.ConnectionId, analyzerId);
    }
}